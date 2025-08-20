use dioxus::prelude::*;
use image::{DynamicImage, Rgb, RgbaImage};
use imgfx::{self, Direction, SortBy};

pub fn or(image: DynamicImage, color: Rgb<u8>, negate: bool) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::or(image, None, None, color, negate);
    Ok(output)
}

pub fn and(image: DynamicImage, color: Rgb<u8>, negate: bool) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::and(image, None, None, color, negate);
    Ok(output)
}

pub fn xor(image: DynamicImage, color: Rgb<u8>, negate: bool) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitwise::xor(image, None, None, color, negate);
    Ok(output)
}

pub fn left(image: DynamicImage, bits: u8, negate: bool) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitshift(image, imgfx::BitshiftDirection::LEFT, None, bits, negate);
    Ok(output)
}

pub fn right(image: DynamicImage, bits: u8, negate: bool) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bitshift(image, imgfx::BitshiftDirection::RIGHT, None, bits, negate);
    Ok(output)
}

pub fn add(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::add(image, None, None, color);
    Ok(output)
}

pub fn sub(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::sub(image, None, None, color, false);
    Ok(output)
}

pub fn mult(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::mult(image, None, None, color);
    Ok(output)
}

pub fn pow(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::pow(image, None, None, color);
    Ok(output)
}

pub fn div(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::div(image, None, None, color);
    Ok(output)
}

pub fn average(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::average(image, None, None, color);
    Ok(output)
}

pub fn screen(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::screen(image, None, None, color);
    Ok(output)
}

pub fn overlay(image: DynamicImage, color: Rgb<u8>) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::overlay(image, None, None, color);
    Ok(output)
}

pub fn bloom(
    image: DynamicImage,
    intensity: f64,
    blur_radius: f64,
    min_threshold: u8,
    max_threshold: Option<u8>,
) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::bloom(image, intensity, blur_radius, min_threshold, max_threshold);
    Ok(output)
}

pub fn sort(
    image: DynamicImage,
    sort_by: SortBy,
    direction: Direction,
    min_threshold: f64,
    max_threshold: f64,
    reversed: bool,
) -> Result<RgbaImage, ServerFnError> {
    let output = imgfx::sort(
        image.into(),
        direction,
        sort_by,
        min_threshold,
        max_threshold,
        reversed,
    );
    Ok(output)
}
