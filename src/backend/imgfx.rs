use dioxus::prelude::*;
use image::{DynamicImage, Rgb, RgbaImage};
use imgfx;

pub fn or(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::or(image, None, None, color, false);
    Ok(output)
}

pub fn and(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::and(image, None, None, color, false);
    Ok(output)
}

pub fn xor(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::xor(image, None, None, color, false);
    Ok(output)
}

pub fn left(image: DynamicImage, bits: u8) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitshift(image, imgfx::BitshiftDirection::LEFT, None, bits, false);
    Ok(output)
}

pub fn right(image: DynamicImage, bits: u8) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitshift(image, imgfx::BitshiftDirection::RIGHT, None, bits, false);
    Ok(output)
}
