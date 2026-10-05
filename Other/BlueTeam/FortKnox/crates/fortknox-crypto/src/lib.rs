use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zeroize::Zeroize;

pub const FORTKNOX_CRYPTO_PROFILE: &str =
    "FortKnox v0.1 local crypto profile: AES-256-GCM, 96-bit random nonces, client-side encryption only.";

const ENVELOPE_VERSION: u8 = 1;
const ALGORITHM: &str = "AES-256-GCM";

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("encryption failed")]
    EncryptionFailed,

    #[error("decryption failed; ciphertext may be corrupted, tampered with, or encrypted with a different key")]
    DecryptionFailed,

    #[error("invalid base64 field: {0}")]
    InvalidBase64(#[from] base64::DecodeError),

    #[error("invalid nonce length")]
    InvalidNonceLength,

    #[error("unsupported encrypted envelope version or algorithm")]
    UnsupportedEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptedEnvelope {
    pub version: u8,
    pub algorithm: String,
    pub nonce_b64: String,
    pub ciphertext_b64: String,
}

#[derive(Debug)]
pub struct SymmetricKey {
    bytes: [u8; 32],
}

impl SymmetricKey {
    pub fn generate() -> Self {
        let key = Aes256Gcm::generate_key(&mut OsRng);

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&key);

        Self { bytes }
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }
}

impl Drop for SymmetricKey {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

pub fn encrypt_message(
    key: &SymmetricKey,
    plaintext: &[u8],
) -> Result<EncryptedEnvelope, CryptoError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key.bytes));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| CryptoError::EncryptionFailed)?;

    Ok(EncryptedEnvelope {
        version: ENVELOPE_VERSION,
        algorithm: ALGORITHM.to_string(),
        nonce_b64: STANDARD_NO_PAD.encode(nonce),
        ciphertext_b64: STANDARD_NO_PAD.encode(ciphertext),
    })
}

pub fn decrypt_message(
    key: &SymmetricKey,
    envelope: &EncryptedEnvelope,
) -> Result<Vec<u8>, CryptoError> {
    if envelope.version != ENVELOPE_VERSION || envelope.algorithm != ALGORITHM {
        return Err(CryptoError::UnsupportedEnvelope);
    }

    let nonce_bytes = STANDARD_NO_PAD.decode(&envelope.nonce_b64)?;
    if nonce_bytes.len() != 12 {
        return Err(CryptoError::InvalidNonceLength);
    }

    let ciphertext = STANDARD_NO_PAD.decode(&envelope.ciphertext_b64)?;

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key.bytes));
    let nonce = Nonce::from_slice(&nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| CryptoError::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = SymmetricKey::generate();
        let plaintext = b"FortKnox test message: client encrypts, server relays, recipient decrypts.";

        let envelope = encrypt_message(&key, plaintext).expect("encryption should work");
        let decrypted = decrypt_message(&key, &envelope).expect("decryption should work");

        assert_eq!(decrypted, plaintext);
        assert_eq!(envelope.version, 1);
        assert_eq!(envelope.algorithm, "AES-256-GCM");
        assert_ne!(envelope.ciphertext_b64, STANDARD_NO_PAD.encode(plaintext));
    }

    #[test]
    fn tampered_ciphertext_fails_decryption() {
        let key = SymmetricKey::generate();
        let plaintext = b"message integrity must be enforced";

        let mut envelope = encrypt_message(&key, plaintext).expect("encryption should work");

        let mut ciphertext = STANDARD_NO_PAD
            .decode(&envelope.ciphertext_b64)
            .expect("ciphertext should be valid base64");

        ciphertext[0] ^= 0xFF;
        envelope.ciphertext_b64 = STANDARD_NO_PAD.encode(ciphertext);

        let result = decrypt_message(&key, &envelope);

        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn wrong_key_fails_decryption() {
        let correct_key = SymmetricKey::generate();
        let wrong_key = SymmetricKey::generate();

        let plaintext = b"wrong keys must not decrypt FortKnox messages";
        let envelope = encrypt_message(&correct_key, plaintext).expect("encryption should work");

        let result = decrypt_message(&wrong_key, &envelope);

        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }
}
