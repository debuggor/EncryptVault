use tauri::State;
use crate::state::AppState;
use crate::paths::{vault_db_path, vault_salt_path};

#[tauri::command]
pub fn reset_master_password(
    current_password: String,
    new_password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let current_key = state
        .vault_key
        .lock()
        .unwrap()
        .ok_or("vault is locked")?;
    settings::master_password::reset_master_password(
        &vault_db_path(),
        &vault_salt_path(),
        &current_password,
        &new_password,
        &current_key,
    )?;
    state.lock();
    Ok(())
}
