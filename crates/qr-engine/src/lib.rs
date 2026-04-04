use image::{DynamicImage, GrayImage, ImageFormat, Luma};
use qrcode::QrCode;
use rqrr::PreparedImage;
use std::io::Cursor;

/// Generate a QR code PNG from a UTF-8 string. Returns raw PNG bytes.
pub fn generate_qr_png(content: &str) -> Result<Vec<u8>, String> {
    let code = QrCode::new(content.as_bytes()).map_err(|e| format!("QR encode: {e}"))?;
    let image: GrayImage = code.render::<Luma<u8>>().quiet_zone(true).build();
    let mut buf = Vec::new();
    DynamicImage::ImageLuma8(image)
        .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
        .map_err(|e| format!("PNG encode: {e}"))?;
    Ok(buf)
}

/// Decode a QR code from image file bytes (PNG, JPEG, etc.).
pub fn decode_bytes(data: &[u8]) -> Result<String, String> {
    let img = image::load_from_memory(data)
        .map_err(|e| format!("image load: {e}"))?
        .to_luma8();
    let mut prepared = PreparedImage::prepare(img);
    let grids = prepared.detect_grids();
    let grid = grids.into_iter().next().ok_or("no QR code found in image")?;
    let (_, content) = grid.decode().map_err(|e| format!("QR decode: {e}"))?;
    Ok(content)
}

/// Decode a QR code from a raw RGBA camera frame.
/// Returns None if no QR code is detected.
pub fn decode_frame(rgba: &[u8], width: u32, height: u32) -> Option<String> {
    let gray = GrayImage::from_fn(width, height, |x, y| {
        let i = ((y * width + x) * 4) as usize;
        let r = rgba[i] as u32;
        let g = rgba[i + 1] as u32;
        let b = rgba[i + 2] as u32;
        Luma([((r * 77 + g * 150 + b * 29) >> 8) as u8])
    });
    let mut prepared = PreparedImage::prepare(gray);
    let grids = prepared.detect_grids();
    let grid = grids.into_iter().next()?;
    let (_, content) = grid.decode().ok()?;
    Some(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_returns_png_bytes() {
        let png = generate_qr_png("https://example.com").unwrap();
        assert_eq!(&png[0..4], b"\x89PNG");
    }

    #[test]
    fn test_roundtrip_generate_and_decode() {
        let original = "Hello, EncryptVault!";
        let png = generate_qr_png(original).unwrap();
        let decoded = decode_bytes(&png).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_decode_invalid_bytes_returns_err() {
        assert!(decode_bytes(b"not an image").is_err());
    }

    #[test]
    fn test_decode_frame_blank_returns_none() {
        let frame = vec![0u8; 100 * 100 * 4];
        assert!(decode_frame(&frame, 100, 100).is_none());
    }
}
