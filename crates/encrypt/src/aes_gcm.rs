use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

/// Encrypt plaintext with a 32-byte key.
/// Output format: [12-byte random nonce][ciphertext + 16-byte GCM tag]
pub fn encrypt_with_key(key: &[u8; 32], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext).expect("encryption failed");
    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(nonce.as_slice());
    out.extend_from_slice(&ciphertext);
    out
}

/// Decrypt data produced by `encrypt_with_key`.
pub fn decrypt_with_key(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 28 {
        // 12-byte nonce + 16-byte tag minimum
        return Err("ciphertext too short".to_string());
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "decryption failed — wrong key or corrupted data".to_string())
}
