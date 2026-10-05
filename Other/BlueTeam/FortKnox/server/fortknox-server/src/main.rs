use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use fortknox_protocol::{RelayEnvelope, FORTKNOX_PROTOCOL_VERSION};
use serde::Serialize;
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::{env, net::SocketAddr, time::Duration};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, FromRow)]
struct RelayMessageRow {
    message_id: Uuid,
    room_id: Uuid,
    sender_device_id: Uuid,
    recipient_device_id: Uuid,
    created_at: DateTime<Utc>,
    encrypted_payload: Value,
}

#[derive(Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
    database: &'static str,
    encryption_policy: &'static str,
}

#[derive(Serialize)]
struct RelayAcceptedResponse {
    response_type: &'static str,
    status: &'static str,
    stored: bool,
    message_id: String,
    room_id: String,
    sender_device_id: String,
    recipient_device_id: String,
    server_plaintext_access: bool,
    note: &'static str,
}

#[derive(Serialize)]
struct RelayRejectedResponse {
    response_type: &'static str,
    status: &'static str,
    reason: String,
    server_plaintext_access: bool,
}

#[derive(Serialize)]
struct BinaryReceivedResponse {
    response_type: &'static str,
    status: &'static str,
    byte_len: usize,
    note: &'static str,
}

#[derive(Serialize)]
struct MessageListResponse {
    response_type: &'static str,
    status: &'static str,
    recipient_device_id: String,
    count: usize,
    server_plaintext_access: bool,
    messages: Vec<RelayEnvelope>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    init_tracing();

    let host = env::var("FORTKNOX_SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("FORTKNOX_SERVER_PORT")
        .unwrap_or_else(|_| "7878".to_string())
        .parse::<u16>()?;

    let database_url = env::var("DATABASE_URL")?;

    let db = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    ensure_schema(&db).await?;

    let app_state = AppState { db };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/ws", get(websocket_handler))
        .route("/messages/{recipient_device_id}", get(list_messages_for_recipient))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("FortKnox server listening on http://{}", addr);
    info!("WebSocket endpoint available at ws://{}/ws", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "fortknox_server=debug,tower_http=debug".into());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn ensure_schema(db: &PgPool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS relay_messages (
            message_id UUID PRIMARY KEY,
            room_id UUID NOT NULL,
            sender_device_id UUID NOT NULL,
            recipient_device_id UUID NOT NULL,
            created_at TIMESTAMPTZ NOT NULL,
            encrypted_payload JSONB NOT NULL,
            received_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_relay_messages_recipient_received
        ON relay_messages (recipient_device_id, received_at DESC)
        "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_relay_messages_room_received
        ON relay_messages (room_id, received_at DESC)
        "#,
    )
    .execute(db)
    .await?;

    Ok(())
}

async fn root() -> &'static str {
    "FortKnox Secure Communications Server"
}

async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let database_status = match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => "online",
        Err(err) => {
            error!("database health check failed: {}", err);
            "offline"
        }
    };

    let status_code = if database_status == "online" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let response = HealthResponse {
        service: "fortknox-server",
        status: if database_status == "online" {
            "online"
        } else {
            "degraded"
        },
        database: database_status,
        encryption_policy: "server stores ciphertext only; plaintext and private keys are client-side only",
    };

    (status_code, Json(response))
}

async fn list_messages_for_recipient(
    State(state): State<AppState>,
    Path(recipient_device_id): Path<Uuid>,
) -> Response {
    match fetch_relay_envelopes_for_recipient(&state.db, recipient_device_id).await {
        Ok(messages) => {
            let response = MessageListResponse {
                response_type: "message_list",
                status: "ok",
                recipient_device_id: recipient_device_id.to_string(),
                count: messages.len(),
                server_plaintext_access: false,
                messages,
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            let response = RelayRejectedResponse {
                response_type: "message_list",
                status: "rejected",
                reason: format!("failed to fetch encrypted messages: {err}"),
                server_plaintext_access: false,
            };

            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

async fn fetch_relay_envelopes_for_recipient(
    db: &PgPool,
    recipient_device_id: Uuid,
) -> Result<Vec<RelayEnvelope>, String> {
    let rows = sqlx::query_as::<_, RelayMessageRow>(
        r#"
        SELECT
            message_id,
            room_id,
            sender_device_id,
            recipient_device_id,
            created_at,
            encrypted_payload
        FROM relay_messages
        WHERE recipient_device_id = $1
        ORDER BY received_at DESC
        LIMIT 100
        "#,
    )
    .bind(recipient_device_id)
    .fetch_all(db)
    .await
    .map_err(|err| err.to_string())?;

    let mut messages = Vec::with_capacity(rows.len());

    for row in rows {
        let encrypted_payload = serde_json::from_value(row.encrypted_payload)
            .map_err(|err| format!("invalid encrypted payload in database: {err}"))?;

        messages.push(RelayEnvelope {
            version: FORTKNOX_PROTOCOL_VERSION,
            message_id: row.message_id,
            room_id: row.room_id,
            sender_device_id: row.sender_device_id,
            recipient_device_id: row.recipient_device_id,
            created_at: row.created_at,
            encrypted_payload,
        });
    }

    Ok(messages)
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    info!("WebSocket client connected");

    while let Some(result) = socket.recv().await {
        match result {
            Ok(Message::Text(text)) => {
                handle_text_message(&mut socket, &state, &text).await;
            }
            Ok(Message::Binary(binary)) => {
                let response = BinaryReceivedResponse {
                    response_type: "binary_received",
                    status: "accepted",
                    byte_len: binary.len(),
                    note: "binary relay support placeholder; future attachment chunks must be encrypted client-side",
                };

                if send_json(&mut socket, &response).await.is_err() {
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket client disconnected");
                break;
            }
            Ok(_) => {}
            Err(err) => {
                error!("WebSocket error: {}", err);
                break;
            }
        }
    }
}

async fn handle_text_message(socket: &mut WebSocket, state: &AppState, text: &str) {
    match serde_json::from_str::<RelayEnvelope>(text) {
        Ok(envelope) => match store_relay_envelope(&state.db, &envelope).await {
            Ok(()) => {
                let response = RelayAcceptedResponse {
                    response_type: "relay_envelope",
                    status: "accepted",
                    stored: true,
                    message_id: envelope.message_id.to_string(),
                    room_id: envelope.room_id.to_string(),
                    sender_device_id: envelope.sender_device_id.to_string(),
                    recipient_device_id: envelope.recipient_device_id.to_string(),
                    server_plaintext_access: false,
                    note: "FortKnox server stored encrypted relay envelope; plaintext remains client-side only",
                };

                let _ = send_json(socket, &response).await;
            }
            Err(err) => {
                error!("failed to store encrypted relay envelope: {}", err);

                let response = RelayRejectedResponse {
                    response_type: "relay_envelope",
                    status: "rejected",
                    reason: format!("database storage failed: {err}"),
                    server_plaintext_access: false,
                };

                let _ = send_json(socket, &response).await;
            }
        },
        Err(err) => {
            let response = RelayRejectedResponse {
                response_type: "relay_envelope",
                status: "rejected",
                reason: format!("invalid RelayEnvelope JSON: {err}"),
                server_plaintext_access: false,
            };

            let _ = send_json(socket, &response).await;
        }
    }
}

async fn store_relay_envelope(db: &PgPool, envelope: &RelayEnvelope) -> Result<(), String> {
    let encrypted_payload =
        serde_json::to_value(&envelope.encrypted_payload).map_err(|err| err.to_string())?;

    sqlx::query(
        r#"
        INSERT INTO relay_messages (
            message_id,
            room_id,
            sender_device_id,
            recipient_device_id,
            created_at,
            encrypted_payload
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (message_id) DO NOTHING
        "#,
    )
    .bind(envelope.message_id)
    .bind(envelope.room_id)
    .bind(envelope.sender_device_id)
    .bind(envelope.recipient_device_id)
    .bind(envelope.created_at)
    .bind(encrypted_payload)
    .execute(db)
    .await
    .map_err(|err| err.to_string())?;

    Ok(())
}

async fn send_json<T: Serialize>(socket: &mut WebSocket, value: &T) -> Result<(), axum::Error> {
    let json = match serde_json::to_string(value) {
        Ok(json) => json,
        Err(err) => {
            format!(
                r#"{{"response_type":"server_error","status":"error","reason":"failed to serialize response: {err}","server_plaintext_access":false}}"#
            )
        }
    };

    socket.send(Message::Text(json.into())).await
}
