pub fn reset_master_password(
    db_path: &str,
    salt_path: &str,
    current_password: &str,
    new_password: &str,
    current_key: &[u8; 32],
) -> Result<(), String> {
    // Step 1: Verify current password matches current_key
    let salt_bytes = std::fs::read(salt_path).map_err(|e| e.to_string())?;
    let salt: [u8; 16] = salt_bytes
        .try_into()
        .map_err(|_| "corrupt salt file".to_string())?;
    let derived = encrypt::derive_key(current_password, &salt)?;
    if derived != *current_key {
        return Err("Current password is incorrect".to_string());
    }

    // Step 2: Load all credentials from the current vault
    let vault = password_vault::Vault::open(db_path, current_key)?;
    let credentials: Vec<password_vault::Credential> =
        vault.list().into_iter().cloned().collect();

    // Step 3: Generate new salt and key
    let new_salt = encrypt::generate_salt();
    let new_key = encrypt::derive_key(new_password, &new_salt)?;

    // Step 4: Re-encrypt all credentials into a temp vault
    let db_tmp = format!("{db_path}.tmp");
    let mut vault_tmp = password_vault::Vault::open(&db_tmp, &new_key)?;
    // Use `update` (not `add`) to preserve original credential IDs.
    // `add` would generate new UUIDs, breaking any external references.
    for cred in credentials {
        vault_tmp.update(cred)?;
    }

    // Step 5: Atomic commit.
    // Salt is renamed first intentionally: if the process crashes after the salt rename
    // but before the DB rename, db.tmp still exists on disk. A user with the new password
    // and the committed new salt can manually rename db.tmp → db to recover. Committing
    // the DB first would permanently lose the new salt (held only in memory), making the
    // new DB unrecoverable.
    let salt_tmp = format!("{salt_path}.tmp");
    std::fs::write(&salt_tmp, new_salt).map_err(|e| e.to_string())?;
    std::fs::rename(&salt_tmp, salt_path).map_err(|e| e.to_string())?;
    std::fs::rename(&db_tmp, db_path).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use encrypt::{derive_key, generate_salt};
    use password_vault::{Credential, Vault};
    use std::env;

    fn temp_paths() -> (String, String) {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let thread_id = format!("{:?}", std::thread::current().id());
        let unique = format!("{nanos}_{}", thread_id.replace(['(', ')', ' '], "_"));
        let db = env::temp_dir()
            .join(format!("settings_test_{unique}.db"))
            .to_string_lossy()
            .to_string();
        let salt = db.replace(".db", ".salt");
        (db, salt)
    }

    fn mk_cred(name: &str) -> Credential {
        Credential {
            id: String::new(),
            name: name.to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
            url: String::new(),
            notes: String::new(),
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn test_reset_success_data_preserved() {
        let (db_path, salt_path) = temp_paths();
        let current_password = "hunter2";
        let new_password = "newpass123";

        // Set up initial vault with salt file and two credentials
        let salt = generate_salt();
        std::fs::write(&salt_path, salt).unwrap();
        let key = derive_key(current_password, &salt).unwrap();
        {
            let mut vault = Vault::open(&db_path, &key).unwrap();
            vault.add(mk_cred("GitHub")).unwrap();
            vault.add(mk_cred("Gmail")).unwrap();
        }

        // Reset the master password
        reset_master_password(&db_path, &salt_path, current_password, new_password, &key)
            .unwrap();

        // Read the new salt and derive new key
        let new_salt_bytes = std::fs::read(&salt_path).unwrap();
        let new_salt: [u8; 16] = new_salt_bytes.try_into().unwrap();
        let new_key = derive_key(new_password, &new_salt).unwrap();

        // Verify new key is different from old key
        assert_ne!(key, new_key);

        // Verify new vault opens with new key and contains both credentials
        let vault = Vault::open(&db_path, &new_key).unwrap();
        let mut names: Vec<&str> = vault.list().iter().map(|c| c.name.as_str()).collect();
        names.sort();
        assert_eq!(names, vec!["GitHub", "Gmail"]);
    }

    #[test]
    fn test_reset_wrong_current_password_fails() {
        let (db_path, salt_path) = temp_paths();
        let current_password = "correct_password";

        let salt = generate_salt();
        std::fs::write(&salt_path, salt).unwrap();
        let key = derive_key(current_password, &salt).unwrap();
        {
            Vault::open(&db_path, &key).unwrap();
        }

        let result =
            reset_master_password(&db_path, &salt_path, "wrong_password", "newpass", &key);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Current password is incorrect");

        // Original salt file is untouched (rename of .tmp never happened)
        let salt_on_disk = std::fs::read(&salt_path).unwrap();
        assert_eq!(salt_on_disk, salt.as_ref());

        // Original vault is still openable with the original key
        let vault = Vault::open(&db_path, &key).unwrap();
        let _ = vault.list(); // just verify it opens without error
    }
}
