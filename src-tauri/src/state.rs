use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub vault_key: Mutex<Option<[u8; 32]>>,
    pub vault: Mutex<Option<()>>,
    pub wallet: Mutex<Option<()>>,
    pub vault_db_path: Mutex<Option<String>>,
}
