mod kdf;
mod aes_gcm;

pub use kdf::{derive_key, generate_salt};
pub use aes_gcm::{encrypt_with_key, decrypt_with_key};

#[cfg(test)]
mod tests {
    use super::*;

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
}
