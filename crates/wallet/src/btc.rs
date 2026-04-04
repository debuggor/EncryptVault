use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    secp256k1::Secp256k1,
    Network,
};
use std::str::FromStr;

/// Derive a BIP-84 P2WPKH (native SegWit "bc1q...") address at m/84'/0'/0'/0/{index}.
pub fn btc_address(seed: &[u8; 64], index: u32) -> Result<String, String> {
    let secp = Secp256k1::new();
    let master = Xpriv::new_master(Network::Bitcoin, seed)
        .map_err(|e| format!("BTC master key: {e}"))?;
    let path = DerivationPath::from_str(&format!("m/84'/0'/0'/0/{index}"))
        .map_err(|e| format!("derivation path: {e}"))?;
    let child = master
        .derive_priv(&secp, &path)
        .map_err(|e| format!("BTC child key: {e}"))?;
    let pubkey = child.to_priv().public_key(&secp);
    let address = bitcoin::Address::p2wpkh(&pubkey, Network::Bitcoin)
        .map_err(|e| format!("p2wpkh: {e}"))?;
    Ok(address.to_string())
}
