use std::sync::Mutex;
use password_vault::Vault;
use wallet::Wallet;

pub struct AppState {
    pub vault_key: Mutex<Option<[u8; 32]>>,
    pub vault: Mutex<Option<Vault>>,
    pub wallet: Mutex<Option<Wallet>>,
    pub vault_db_path: Mutex<Option<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            vault_key: Mutex::new(None),
            vault: Mutex::new(None),
            wallet: Mutex::new(None),
            vault_db_path: Mutex::new(None),
        }
    }
}

impl AppState {
    pub fn is_unlocked(&self) -> bool {
        self.vault_key.lock().unwrap().is_some()
    }

    pub fn lock(&self) {
        *self.vault_key.lock().unwrap() = None;
        *self.vault.lock().unwrap() = None;
        *self.wallet.lock().unwrap() = None;
    }
}
