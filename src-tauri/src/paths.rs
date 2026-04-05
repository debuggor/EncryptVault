pub fn vault_db_path() -> String {
    let mut path = dirs_next::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("EncryptVault");
    std::fs::create_dir_all(&path).ok();
    path.push("vault.db");
    path.to_string_lossy().to_string()
}

pub fn vault_salt_path() -> String {
    vault_db_path().replace(".db", ".salt")
}
