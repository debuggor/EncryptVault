use crate::paths::{vault_db_path, vault_salt_path};
use crate::state::AppState;
use encrypt::{derive_key, generate_salt};
use password_vault::{Credential, Vault};
use tauri::State;

#[tauri::command]
pub fn is_unlocked(state: State<'_, AppState>) -> bool {
    state.is_unlocked()
}

#[tauri::command]
pub fn unlock_vault(password: String, state: State<'_, AppState>) -> Result<(), String> {
    let salt_path = vault_salt_path();
    let salt: [u8; 16] = if std::path::Path::new(&salt_path).exists() {
        let bytes = std::fs::read(&salt_path).map_err(|e| e.to_string())?;
        bytes
            .try_into()
            .map_err(|_| "corrupt salt file".to_string())?
    } else {
        let s = generate_salt();
        // Write atomically: temp file + rename to avoid a corrupt salt on crash
        let tmp = format!("{salt_path}.tmp");
        std::fs::write(&tmp, s).map_err(|e| e.to_string())?;
        std::fs::rename(&tmp, &salt_path).map_err(|e| e.to_string())?;
        s
    };

    let key = derive_key(&password, &salt)?;
    let db_path = vault_db_path();
    let vault = Vault::open(&db_path, &key).map_err(|e| {
        state.lock();
        e
    })?;

    *state.vault_key.lock().unwrap() = Some(key);
    *state.vault.lock().unwrap() = Some(vault);
    Ok(())
}

#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>) {
    state.lock();
}

#[tauri::command]
pub fn add_credential(
    name: String,
    username: String,
    password: String,
    url: String,
    notes: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_mut().ok_or("vault is locked")?;
    vault.add(Credential {
        id: String::new(),
        name,
        username,
        password,
        url,
        notes,
        created_at: 0,
        updated_at: 0,
    })
}

#[tauri::command]
pub fn list_credentials(state: State<'_, AppState>) -> Result<Vec<Credential>, String> {
    let vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_ref().ok_or("vault is locked")?;
    Ok(vault.list().into_iter().cloned().collect())
}

#[tauri::command]
pub fn update_credential(
    id: String,
    name: String,
    username: String,
    password: String,
    url: String,
    notes: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_mut().ok_or("vault is locked")?;
    let existing_created_at = vault
        .list()
        .into_iter()
        .find(|c| c.id == id)
        .ok_or("credential not found")?
        .created_at;
    vault.update(Credential {
        id,
        name,
        username,
        password,
        url,
        notes,
        created_at: existing_created_at,
        updated_at: 0,
    })
}

#[tauri::command]
pub fn delete_credential(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_mut().ok_or("vault is locked")?;
    vault.delete(&id)
}

#[tauri::command]
pub fn search_credentials(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<Credential>, String> {
    let vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_ref().ok_or("vault is locked")?;
    Ok(vault.search(&query).into_iter().cloned().collect())
}
