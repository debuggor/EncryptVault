use tauri::State;
use crate::state::AppState;
use wallet::{EthTx, Wallet};

#[tauri::command]
pub fn setup_wallet(state: State<'_, AppState>) -> Result<String, String> {
    let mut wallet_guard = state.wallet.lock().unwrap();
    if wallet_guard.is_some() {
        return Err("wallet already loaded".to_string());
    }
    let (wallet, mnemonic) = Wallet::generate();
    *wallet_guard = Some(wallet);
    Ok(mnemonic)
}

#[tauri::command]
pub fn import_wallet(mnemonic: String, state: State<'_, AppState>) -> Result<(), String> {
    let wallet = Wallet::from_mnemonic(&mnemonic)?;
    *state.wallet.lock().unwrap() = Some(wallet);
    Ok(())
}

#[tauri::command]
pub fn derive_eth_address(index: u32, state: State<'_, AppState>) -> Result<String, String> {
    let guard = state.wallet.lock().unwrap();
    let wallet = guard.as_ref().ok_or("wallet not loaded")?;
    wallet.eth_address(index)
}

#[tauri::command]
pub fn derive_btc_address(index: u32, state: State<'_, AppState>) -> Result<String, String> {
    let guard = state.wallet.lock().unwrap();
    let wallet = guard.as_ref().ok_or("wallet not loaded")?;
    wallet.btc_address(index)
}

#[tauri::command]
pub fn sign_eth_tx(
    index: u32,
    tx: EthTx,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let guard = state.wallet.lock().unwrap();
    let wallet = guard.as_ref().ok_or("wallet not loaded")?;
    wallet.sign_eth_tx(index, &tx)
}
