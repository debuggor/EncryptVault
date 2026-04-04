mod kdf;
mod aes_gcm;

pub use kdf::{derive_key, generate_salt};
pub use aes_gcm::{encrypt_with_key, decrypt_with_key};

use base64::{engine::general_purpose::STANDARD as B64, Engine};

/// Encrypt a UTF-8 string with a passphrase.
/// Wire format (base64-encoded): [16-byte salt][12-byte nonce][ciphertext+tag]
pub fn encrypt_text(passphrase: &str, plaintext: &str) -> Result<String, String> {
    let salt = generate_salt();
    let key = derive_key(passphrase, &salt)?;
    let enc = encrypt_with_key(&key, plaintext.as_bytes())?;
    let mut wire = Vec::with_capacity(16 + enc.len());
    wire.extend_from_slice(&salt);
    wire.extend_from_slice(&enc);
    Ok(B64.encode(wire))
}

/// Decrypt a string produced by `encrypt_text`.
pub fn decrypt_text(passphrase: &str, encoded: &str) -> Result<String, String> {
    let wire = B64.decode(encoded).map_err(|_| "invalid base64".to_string())?;
    if wire.len() < 16 {
        return Err("data too short".to_string());
    }
    let (salt_slice, rest) = wire.split_at(16);
    let salt: [u8; 16] = salt_slice.try_into().unwrap();
    let key = derive_key(passphrase, &salt)?;
    let pt = decrypt_with_key(&key, rest)?;
    String::from_utf8(pt).map_err(|_| "decrypted data is not valid UTF-8".to_string())
}

/// Encrypt raw bytes with a passphrase.
/// Wire format: [16-byte salt][12-byte nonce][ciphertext+tag]
pub fn encrypt_bytes_with_passphrase(passphrase: &str, data: &[u8]) -> Result<Vec<u8>, String> {
    let salt = generate_salt();
    let key = derive_key(passphrase, &salt)?;
    let enc = encrypt_with_key(&key, data)?;
    let mut wire = Vec::with_capacity(16 + enc.len());
    wire.extend_from_slice(&salt);
    wire.extend_from_slice(&enc);
    Ok(wire)
}

/// Decrypt bytes produced by `encrypt_bytes_with_passphrase`.
pub fn decrypt_bytes_with_passphrase(passphrase: &str, data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 16 {
        return Err("data too short".to_string());
    }
    let (salt_slice, rest) = data.split_at(16);
    let salt: [u8; 16] = salt_slice.try_into().unwrap();
    let key = derive_key(passphrase, &salt)?;
    decrypt_with_key(&key, rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- primitives (from Task 3) ---

    #[test]
    fn test_derive_key_is_deterministic() {
        let salt = [1u8; 16];
        assert_eq!(derive_key("password", &salt).unwrap(), derive_key("password", &salt).unwrap());
    }

    #[test]
    fn test_derive_key_differs_by_password() {
        let salt = [1u8; 16];
        assert_ne!(derive_key("a", &salt).unwrap(), derive_key("b", &salt).unwrap());
    }

    #[test]
    fn test_derive_key_differs_by_salt() {
        assert_ne!(derive_key("pw", &[1u8; 16]).unwrap(), derive_key("pw", &[2u8; 16]).unwrap());
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [42u8; 32];
        let ct = encrypt_with_key(&key, b"hello vault").unwrap();
        let pt = decrypt_with_key(&key, &ct).unwrap();
        assert_eq!(pt, b"hello vault");
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let ct = encrypt_with_key(&[1u8; 32], b"secret").unwrap();
        assert!(decrypt_with_key(&[2u8; 32], &ct).is_err());
    }

    #[test]
    fn test_ciphertext_different_each_time() {
        let key = [0u8; 32];
        assert_ne!(encrypt_with_key(&key, b"same").unwrap(), encrypt_with_key(&key, b"same").unwrap());
    }

    // --- high-level API (Task 4) ---

    #[test]
    fn test_encrypt_text_roundtrip() {
        let ct = encrypt_text("passphrase", "hello world").unwrap();
        assert_eq!(decrypt_text("passphrase", &ct).unwrap(), "hello world");
    }

    #[test]
    fn test_decrypt_text_wrong_passphrase_fails() {
        let ct = encrypt_text("correct", "secret").unwrap();
        assert!(decrypt_text("wrong", &ct).is_err());
    }

    #[test]
    fn test_encrypt_text_different_each_time() {
        let ct1 = encrypt_text("pw", "same").unwrap();
        let ct2 = encrypt_text("pw", "same").unwrap();
        assert_ne!(ct1, ct2);
    }

    #[test]
    fn test_encrypt_bytes_roundtrip() {
        let data = vec![0u8, 1, 2, 3, 255];
        let ct = encrypt_bytes_with_passphrase("pw", &data).unwrap();
        assert_eq!(decrypt_bytes_with_passphrase("pw", &ct).unwrap(), data);
    }

    #[test]
    fn test_decrypt_bytes_wrong_passphrase_fails() {
        let ct = encrypt_bytes_with_passphrase("correct", b"data").unwrap();
        assert!(decrypt_bytes_with_passphrase("wrong", &ct).is_err());
    }
}
