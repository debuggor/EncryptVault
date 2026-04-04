use coins_bip32::prelude::*;
use k256::ecdsa::SigningKey;
use tiny_keccak::{Hasher, Keccak};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EthTx {
    pub chain_id: u64,
    pub nonce: u64,
    pub to: String,
    pub value: String,     // decimal string, wei
    pub gas_price: String, // decimal string, wei
    pub gas_limit: u64,
    pub data: String,      // hex string, "0x..." or "0x"
}

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut h = Keccak::v256();
    h.update(data);
    let mut out = [0u8; 32];
    h.finalize(&mut out);
    out
}

fn derive_signing_key(seed: &[u8; 64], path: &str) -> Result<SigningKey, String> {
    let xpriv = XPriv::root_from_seed(seed, None)
        .map_err(|e| format!("root key derivation: {e}"))?;
    let child: XPriv = xpriv
        .derive_path(path)
        .map_err(|e| format!("path derivation: {e}"))?;
    let signing_key_ref: &k256::ecdsa::SigningKey = child.as_ref();
    Ok(signing_key_ref.clone())
}

/// Derive the ETH address at BIP-44 path m/44'/60'/0'/0/{index}.
pub fn eth_address(seed: &[u8; 64], index: u32) -> Result<String, String> {
    let signing_key = derive_signing_key(seed, &format!("m/44'/60'/0'/0/{index}"))?;
    let verifying_key = signing_key.verifying_key();
    let encoded = verifying_key.to_encoded_point(false);
    let pubkey_bytes = &encoded.as_bytes()[1..]; // strip 0x04 prefix
    let hash = keccak256(pubkey_bytes);
    let addr_bytes = &hash[12..]; // last 20 bytes
    Ok(format!("0x{}", hex::encode(addr_bytes)))
}

/// Sign an EIP-155 transaction offline. Returns "0x..." hex of the signed raw tx.
pub fn sign_eth_tx(seed: &[u8; 64], index: u32, tx: &EthTx) -> Result<String, String> {
    let signing_key = derive_signing_key(seed, &format!("m/44'/60'/0'/0/{index}"))?;

    let value_bytes = decimal_to_be_bytes_minimal(&tx.value)?;
    let gas_price_bytes = decimal_to_be_bytes_minimal(&tx.gas_price)?;
    let to_bytes = hex_to_bytes(tx.to.strip_prefix("0x").unwrap_or(&tx.to))?;
    if to_bytes.len() != 20 {
        return Err(format!("invalid 'to' address: expected 20 bytes, got {}", to_bytes.len()));
    }
    let data_bytes = hex_to_bytes(tx.data.strip_prefix("0x").unwrap_or(""))?;

    // EIP-155 signing payload: RLP([nonce, gasPrice, gasLimit, to, value, data, chainId, 0, 0])
    let mut stream = rlp::RlpStream::new_list(9);
    stream.append(&tx.nonce);
    stream.append(&gas_price_bytes.as_slice());
    stream.append(&tx.gas_limit);
    stream.append(&to_bytes.as_slice());
    stream.append(&value_bytes.as_slice());
    stream.append(&data_bytes.as_slice());
    stream.append(&tx.chain_id);
    stream.append(&0u8);
    stream.append(&0u8);
    let hash = keccak256(&stream.out());

    let (sig, recovery_id) = signing_key
        .sign_prehash_recoverable(&hash)
        .map_err(|e| format!("signing failed: {e}"))?;

    let r_bytes = sig.r().to_bytes();
    let s_bytes = sig.s().to_bytes();
    let v = tx.chain_id * 2 + 35 + recovery_id.to_byte() as u64;

    // Final signed tx: RLP([nonce, gasPrice, gasLimit, to, value, data, v, r, s])
    let mut signed = rlp::RlpStream::new_list(9);
    signed.append(&tx.nonce);
    signed.append(&gas_price_bytes.as_slice());
    signed.append(&tx.gas_limit);
    signed.append(&to_bytes.as_slice());
    signed.append(&value_bytes.as_slice());
    signed.append(&data_bytes.as_slice());
    signed.append(&v);
    signed.append(&r_bytes.as_slice());
    signed.append(&s_bytes.as_slice());

    Ok(format!("0x{}", hex::encode(signed.out())))
}

/// Parse a decimal string into a minimal big-endian byte vector (no leading zeros).
fn decimal_to_be_bytes_minimal(s: &str) -> Result<Vec<u8>, String> {
    let n: u128 = s.parse().map_err(|_| format!("invalid number: {s}"))?;
    if n == 0 {
        return Ok(vec![]);
    }
    let bytes = n.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(0);
    Ok(bytes[start..].to_vec())
}

fn hex_to_bytes(s: &str) -> Result<Vec<u8>, String> {
    if s.is_empty() {
        return Ok(vec![]);
    }
    hex::decode(s).map_err(|e| format!("invalid hex: {e}"))
}
