#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod commands;

use state::AppState;
use commands::{
    encrypt_cmd::{cmd_decrypt_file, cmd_decrypt_text, cmd_encrypt_file, cmd_encrypt_text},
    qr_cmd::{cmd_decode_qr_file, cmd_decode_qr_frame, cmd_generate_qr},
    vault_cmd::{
        add_credential, delete_credential, is_unlocked, list_credentials, lock_vault,
        search_credentials, unlock_vault, update_credential,
    },
    wallet_cmd::{derive_btc_address, derive_eth_address, import_wallet, setup_wallet, sign_eth_tx},
};

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            is_unlocked,
            unlock_vault,
            lock_vault,
            add_credential,
            list_credentials,
            update_credential,
            delete_credential,
            search_credentials,
            cmd_encrypt_text,
            cmd_decrypt_text,
            cmd_encrypt_file,
            cmd_decrypt_file,
            setup_wallet,
            import_wallet,
            derive_eth_address,
            derive_btc_address,
            sign_eth_tx,
            cmd_generate_qr,
            cmd_decode_qr_file,
            cmd_decode_qr_frame,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
