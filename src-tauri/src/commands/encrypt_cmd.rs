use encrypt::{decrypt_bytes_with_passphrase, decrypt_text, encrypt_bytes_with_passphrase, encrypt_text};

#[tauri::command]
pub fn cmd_encrypt_text(passphrase: String, plaintext: String) -> Result<String, String> {
    encrypt_text(&passphrase, &plaintext)
}

#[tauri::command]
pub fn cmd_decrypt_text(passphrase: String, ciphertext: String) -> Result<String, String> {
    decrypt_text(&passphrase, &ciphertext)
}

#[tauri::command]
pub fn cmd_encrypt_file(passphrase: String, file_path: String) -> Result<String, String> {
    let data = std::fs::read(&file_path).map_err(|e| format!("read file: {e}"))?;
    let encrypted = encrypt_bytes_with_passphrase(&passphrase, &data)?;
    let out_path = format!("{}.enc", file_path);
    let tmp_path = format!("{}.enc.tmp", file_path);
    std::fs::write(&tmp_path, &encrypted).map_err(|e| format!("write tmp: {e}"))?;
    std::fs::rename(&tmp_path, &out_path).map_err(|e| format!("rename: {e}"))?;
    Ok(out_path)
}

#[tauri::command]
pub fn cmd_decrypt_file(passphrase: String, file_path: String) -> Result<String, String> {
    let data = std::fs::read(&file_path).map_err(|e| format!("read file: {e}"))?;
    let decrypted = decrypt_bytes_with_passphrase(&passphrase, &data)?;
    let out_path = file_path
        .strip_suffix(".enc")
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}.dec", file_path));
    let tmp_path = format!("{}.tmp", out_path);
    std::fs::write(&tmp_path, &decrypted).map_err(|e| format!("write tmp: {e}"))?;
    std::fs::rename(&tmp_path, &out_path).map_err(|e| format!("rename: {e}"))?;
    Ok(out_path)
}
