use base64::{engine::general_purpose::STANDARD as B64, Engine};
use qr_engine::{decode_bytes, decode_frame, generate_qr_png};

#[tauri::command]
pub fn cmd_generate_qr(content: String) -> Result<String, String> {
    let png = generate_qr_png(&content)?;
    Ok(format!("data:image/png;base64,{}", B64.encode(png)))
}

#[tauri::command]
pub fn cmd_decode_qr_file(file_path: String) -> Result<String, String> {
    let data = std::fs::read(&file_path).map_err(|e| format!("read file: {e}"))?;
    decode_bytes(&data)
}

#[tauri::command]
pub fn cmd_decode_qr_frame(rgba: Vec<u8>, width: u32, height: u32) -> Option<String> {
    decode_frame(&rgba, width, height)
}
