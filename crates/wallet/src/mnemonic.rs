use bip39::{Language, Mnemonic};

/// Generate a random 12-word BIP-39 mnemonic.
pub fn generate_mnemonic() -> String {
    Mnemonic::generate_in(Language::English, 12)
        .expect("mnemonic generation failed")
        .to_string()
}

/// Convert a mnemonic phrase to a 64-byte BIP-39 seed (no passphrase).
pub fn mnemonic_to_seed(phrase: &str) -> Result<[u8; 64], String> {
    let mnemonic = Mnemonic::parse_in(Language::English, phrase)
        .map_err(|e| format!("invalid mnemonic: {e}"))?;
    Ok(mnemonic.to_seed(""))
}
