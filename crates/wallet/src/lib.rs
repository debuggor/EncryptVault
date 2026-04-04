mod mnemonic;
mod eth;
mod btc;

pub use eth::EthTx;
pub use mnemonic::{generate_mnemonic, mnemonic_to_seed};

pub struct Wallet {
    seed: [u8; 64],
}

impl Wallet {
    /// Generate a new wallet. Returns (Wallet, mnemonic_phrase).
    pub fn generate() -> (Self, String) {
        let phrase = generate_mnemonic();
        let seed = mnemonic_to_seed(&phrase).expect("generated mnemonic is always valid");
        (Self { seed }, phrase)
    }

    pub fn from_mnemonic(phrase: &str) -> Result<Self, String> {
        let seed = mnemonic_to_seed(phrase)?;
        Ok(Self { seed })
    }

    pub fn eth_address(&self, index: u32) -> Result<String, String> {
        eth::eth_address(&self.seed, index)
    }

    pub fn btc_address(&self, index: u32) -> Result<String, String> {
        btc::btc_address(&self.seed, index)
    }

    pub fn sign_eth_tx(&self, index: u32, tx: &EthTx) -> Result<String, String> {
        eth::sign_eth_tx(&self.seed, index, tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Standard BIP-39 test mnemonic (all "abandon" + "about")
    const TEST_MNEMONIC: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn test_generate_mnemonic_12_words() {
        let phrase = generate_mnemonic();
        assert_eq!(phrase.split_whitespace().count(), 12);
    }

    #[test]
    fn test_generate_different_each_time() {
        assert_ne!(generate_mnemonic(), generate_mnemonic());
    }

    #[test]
    fn test_invalid_mnemonic_fails() {
        assert!(Wallet::from_mnemonic("not valid words here at all twelve").is_err());
    }

    #[test]
    fn test_eth_address_deterministic() {
        let w = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let a1 = w.eth_address(0).unwrap();
        let a2 = w.eth_address(0).unwrap();
        assert_eq!(a1, a2);
        assert!(a1.starts_with("0x"));
        assert_eq!(a1.len(), 42);
    }

    #[test]
    fn test_eth_address_index_differs() {
        let w = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        assert_ne!(w.eth_address(0).unwrap(), w.eth_address(1).unwrap());
    }

    #[test]
    fn test_btc_address_deterministic() {
        let w = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let a = w.btc_address(0).unwrap();
        assert_eq!(a, w.btc_address(0).unwrap());
        assert!(a.starts_with("bc1q"), "expected bc1q address, got: {a}");
    }

    #[test]
    fn test_sign_eth_tx_returns_hex() {
        let w = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let tx = EthTx {
            chain_id: 1,
            nonce: 0,
            to: "0xd3CdA913deB6f4967b2Ef3aa68f5A843aFbFB950".to_string(),
            value: "1000000000000000000".to_string(),
            gas_price: "20000000000".to_string(),
            gas_limit: 21000,
            data: "0x".to_string(),
        };
        let signed = w.sign_eth_tx(0, &tx).unwrap();
        assert!(signed.starts_with("0x"), "signed tx should start with 0x");
        assert!(signed.len() > 20, "signed tx too short");
    }
}
