use chrono::{DateTime, Utc};
use fortknox_crypto::{
    decrypt_message, encrypt_message, CryptoError, EncryptedEnvelope, SymmetricKey,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub const FORTKNOX_PROTOCOL_VERSION: u8 = 1;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("crypto error: {0}")]
    Crypto(#[from] CryptoError),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("unsupported FortKnox protocol version")]
    UnsupportedVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageKind {
    Text,
    System,
    AttachmentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaintextChatMessage {
    pub version: u8,
    pub kind: MessageKind,
    pub room_id: Uuid,
    pub sender_device_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelayEnvelope {
    pub version: u8,
    pub message_id: Uuid,
    pub room_id: Uuid,
    pub sender_device_id: Uuid,
    pub recipient_device_id: Uuid,
    pub created_at: DateTime<Utc>,

    /// This is the only message content the server should see.
    /// The server must never receive plaintext or private keys.
    pub encrypted_payload: EncryptedEnvelope,
}

pub fn encrypt_chat_message_for_relay(
    key: &SymmetricKey,
    room_id: Uuid,
    sender_device_id: Uuid,
    recipient_device_id: Uuid,
    body: impl Into<String>,
) -> Result<RelayEnvelope, ProtocolError> {
    let created_at = Utc::now();

    let plaintext = PlaintextChatMessage {
        version: FORTKNOX_PROTOCOL_VERSION,
        kind: MessageKind::Text,
        room_id,
        sender_device_id,
        created_at,
        body: body.into(),
    };

    let plaintext_bytes = serde_json::to_vec(&plaintext)?;
    let encrypted_payload = encrypt_message(key, &plaintext_bytes)?;

    Ok(RelayEnvelope {
        version: FORTKNOX_PROTOCOL_VERSION,
        message_id: Uuid::new_v4(),
        room_id,
        sender_device_id,
        recipient_device_id,
        created_at,
        encrypted_payload,
    })
}

pub fn decrypt_relay_envelope(
    key: &SymmetricKey,
    envelope: &RelayEnvelope,
) -> Result<PlaintextChatMessage, ProtocolError> {
    if envelope.version != FORTKNOX_PROTOCOL_VERSION {
        return Err(ProtocolError::UnsupportedVersion);
    }

    let plaintext_bytes = decrypt_message(key, &envelope.encrypted_payload)?;
    let plaintext: PlaintextChatMessage = serde_json::from_slice(&plaintext_bytes)?;

    if plaintext.version != FORTKNOX_PROTOCOL_VERSION {
        return Err(ProtocolError::UnsupportedVersion);
    }

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_encrypts_and_decrypts_chat_message() {
        let key = SymmetricKey::generate();

        let room_id = Uuid::new_v4();
        let sender_device_id = Uuid::new_v4();
        let recipient_device_id = Uuid::new_v4();

        let relay = encrypt_chat_message_for_relay(
            &key,
            room_id,
            sender_device_id,
            recipient_device_id,
            "FortKnox protocol test: ciphertext only.",
        )
        .expect("message should encrypt into relay envelope");

        assert_eq!(relay.version, FORTKNOX_PROTOCOL_VERSION);
        assert_eq!(relay.room_id, room_id);
        assert_eq!(relay.sender_device_id, sender_device_id);
        assert_eq!(relay.recipient_device_id, recipient_device_id);

        let serialized_for_server =
            serde_json::to_string(&relay).expect("relay envelope should serialize");

        assert!(serialized_for_server.contains("encrypted_payload"));
        assert!(!serialized_for_server.contains("FortKnox protocol test: ciphertext only."));

        let decrypted =
            decrypt_relay_envelope(&key, &relay).expect("recipient should decrypt message");

        assert_eq!(decrypted.version, FORTKNOX_PROTOCOL_VERSION);
        assert_eq!(decrypted.kind, MessageKind::Text);
        assert_eq!(decrypted.room_id, room_id);
        assert_eq!(decrypted.sender_device_id, sender_device_id);
        assert_eq!(decrypted.body, "FortKnox protocol test: ciphertext only.");
    }

    #[test]
    fn protocol_detects_tampered_payload() {
        let key = SymmetricKey::generate();

        let mut relay = encrypt_chat_message_for_relay(
            &key,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            "tamper detection test",
        )
        .expect("message should encrypt");

        relay.encrypted_payload.ciphertext_b64.push('A');

        let result = decrypt_relay_envelope(&key, &relay);

        assert!(result.is_err());
    }

    #[test]
    fn protocol_rejects_wrong_key() {
        let correct_key = SymmetricKey::generate();
        let wrong_key = SymmetricKey::generate();

        let relay = encrypt_chat_message_for_relay(
            &correct_key,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            "wrong key rejection test",
        )
        .expect("message should encrypt");

        let result = decrypt_relay_envelope(&wrong_key, &relay);

        assert!(result.is_err());
    }
}
