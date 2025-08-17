use base64::Engine;
use image::{DynamicImage, ImageFormat, RgbaImage};
use std::io::Cursor;

/// Convert RgbaImage to a data URL that can be used in Dioxus
pub fn rgba_image_to_data_url(
    rgba_image: &RgbaImage,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create a buffer to hold the encoded image data
    let mut buffer = Cursor::new(Vec::new());

    // Convert RgbaImage to DynamicImage for encoding
    let dynamic_image = DynamicImage::ImageRgba8(rgba_image.clone());

    // Encode as PNG (preserves alpha channel)
    dynamic_image.write_to(&mut buffer, ImageFormat::Png)?;

    // Get the encoded bytes
    let image_bytes = buffer.into_inner();

    // Encode to base64
    let base64_string = base64::engine::general_purpose::STANDARD.encode(&image_bytes);

    // Create data URL
    let data_url = format!("data:image/png;base64,{}", base64_string);

    Ok(data_url)
}

/// Create a preview data URL from a DynamicImage (for showing original image)
pub fn create_preview_data_url(
    dynamic_image: &DynamicImage,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(Vec::new());

    // Convert to RGB if it's RGBA (JPEG doesn't support alpha channel)
    let image_for_encoding = match dynamic_image {
        DynamicImage::ImageRgba8(_) => {
            // Convert RGBA to RGB
            DynamicImage::ImageRgb8(dynamic_image.to_rgb8())
        }
        _ => dynamic_image.clone(),
    };

    // Encode as JPEG for smaller preview size
    image_for_encoding.write_to(&mut buffer, ImageFormat::Jpeg)?;

    let image_bytes = buffer.into_inner();
    let base64_string = base64::engine::general_purpose::STANDARD.encode(&image_bytes);
    let data_url = format!("data:image/jpeg;base64,{}", base64_string);

    Ok(data_url)
}

/// Check if a filename represents an image file
pub fn is_image_file(filename: &str) -> bool {
    let extension = filename.split('.').last().unwrap_or("").to_lowercase();
    matches!(
        extension.as_str(),
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp"
    )
}
