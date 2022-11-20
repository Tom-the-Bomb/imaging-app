//! File containing helper functions used by the
//! individual processing functions for endpoints in `functions.rs`
use base64::encode;
use photon_rs::{native::image_to_bytes, base64_to_image, PhotonImage};
use ril::prelude::*;

/// helper function for lego to colorize the lego brick
/// with each pixel's color in the image
pub fn colorize_lego_band(image: Image<L>, value: i32) -> Image<L> {
    image.map_pixels(|p| {
        let p = p.value() as i32;

        let mut value = if p < 33 {
            value - 100
        } else if p > 233 {
            value + 100
        } else {
            value - 133 + p
        };

        if value < 0 {
            value = 0;
        } else if value > 255 {
            value = 255;
        }

        L::new(value as u8)
    })
}

/// resizes an image to a certain size, using the longest side, maintains aspect ratio
pub fn resize_to(image: Image<Rgba>, size: u32) -> Image<Rgba> {
    let (w, h) = image.dimensions();
    let (width, height) =
        if w > h {
            (size, ((size as f32 / w as f32) * h as f32).ceil() as u32)
        } else {
            (((size as f32 / h as f32) * w as f32).ceil() as u32, size)
        };

    image.resized(width, height, ResizeAlgorithm::Bilinear)
}

pub fn to_photon(image: Image<Rgba>) -> ril::Result<PhotonImage> {
    let mut buffer = Vec::<u8>::new();
    image.encode(ImageFormat::Png, &mut buffer)?;
    let base64 = encode(buffer);

    Ok(base64_to_image(base64.as_str()))
}

#[inline]
pub fn to_ril(image: PhotonImage) -> ril::Result<Image<Rgba>> {
    Image::from_bytes_inferred(
        image_to_bytes(image)
    )
}