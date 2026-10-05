use fortknox_crypto::SymmetricKey;
use fortknox_protocol::{decrypt_relay_envelope, encrypt_chat_message_for_relay, RelayEnvelope};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct MessageListResponse {
    response_type: String,
    status: String,
    recipient_device_id: String,
    count: usize,
    server_plaintext_access: bool,
    messages: Vec<RelayEnvelope>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let websocket_url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "ws://127.0.0.1:7878/ws".to_string());

    let http_base_url = std::env::var("FORTKNOX_HTTP_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:7878".to_string());

    println!("FortKnox Dev Client");
    println!("Connecting to WebSocket: {websocket_url}");
    println!("HTTP base URL: {http_base_url}");

    let key = SymmetricKey::generate();

    let room_id = Uuid::new_v4();
    let sender_device_id = Uuid::new_v4();
    let recipient_device_id = Uuid::new_v4();

    let plaintext = "FortKnox retrieval test: server stores ciphertext, recipient fetches ciphertext, client decrypts locally.";

    let relay = encrypt_chat_message_for_relay(
        &key,
        room_id,
        sender_device_id,
        recipient_device_id,
        plaintext,
    )?;

    let outbound_json = serde_json::to_string(&relay)?;

    if outbound_json.contains(plaintext) {
        return Err("security failure: plaintext appeared in outbound server JSON".into());
    }

    println!("Generated encrypted RelayEnvelope");
    println!("Message ID: {}", relay.message_id);
    println!("Room ID: {}", relay.room_id);
    println!("Sender device: {}", relay.sender_device_id);
    println!("Recipient device: {}", relay.recipient_device_id);
    println!("Plaintext present in outbound JSON: false");
    println!("Outbound JSON byte length: {}", outbound_json.len());

    let (mut socket, _response) = connect_async(&websocket_url).await?;

    socket.send(Message::Text(outbound_json.into())).await?;

    if let Some(message) = socket.next().await {
        match message? {
            Message::Text(text) => {
                println!("Server store response:");
                println!("{text}");
            }
            other => {
                println!("Received non-text server response: {other:?}");
            }
        }
    }

    socket.close(None).await?;

    let fetch_url = format!(
        "{}/messages/{}",
        http_base_url.trim_end_matches('/'),
        recipient_device_id
    );

    println!("Fetching encrypted mailbox from:");
    println!("{fetch_url}");

    let http = reqwest::Client::new();
    let response_text = http
        .get(fetch_url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    if response_text.contains(plaintext) {
        return Err("security failure: plaintext appeared in server retrieval response".into());
    }

    let message_list: MessageListResponse = serde_json::from_str(&response_text)?;

    println!("Mailbox response type: {}", message_list.response_type);
    println!("Mailbox status: {}", message_list.status);
    println!("Mailbox recipient: {}", message_list.recipient_device_id);
    println!("Fetched encrypted message count: {}", message_list.count);
    println!(
        "Server plaintext access during fetch: {}",
        message_list.server_plaintext_access
    );
    println!("Plaintext present in retrieval JSON: false");

    let fetched_envelope = message_list
        .messages
        .iter()
        .find(|message| message.message_id == relay.message_id)
        .ok_or("sent message was not found in fetched encrypted mailbox")?;

    let decrypted = decrypt_relay_envelope(&key, fetched_envelope)?;

    println!("Local decrypt proof after mailbox retrieval:");
    println!("{}", decrypted.body);

    Ok(())
}
