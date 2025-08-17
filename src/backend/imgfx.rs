use clap::builder::styling::RgbColor;
use dioxus::prelude::*;
use image::{DynamicImage, ImageBuffer, RgbaImage};
use imgfx;

pub fn or(image: DynamicImage, color: RgbColor) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::or(image, None, None, color, false);

    Ok(output)
}

pub fn and(image: DynamicImage, color: RgbColor) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::and(image, None, None, color, false);

    Ok(output)
}

pub fn xor(image: DynamicImage, color: RgbColor) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::xor(image, None, None, color, false);

    Ok(output)
}
