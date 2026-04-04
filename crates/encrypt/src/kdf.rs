use argon2::{Algorithm, Argon2, Params, Version};
use rand::RngCore;

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

/// Derive a 32-byte key from password + salt using Argon2id.
/// Params: m=65536 (64 MiB), t=3 iterations, p=4 parallelism.
pub fn derive_key(password: &str, salt: &[u8; 16]) -> [u8; 32] {
    let params = Params::new(65536, 3, 4, Some(32)).expect("valid argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("argon2 key derivation failed");
    key
}
