use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use crate::models::*;
use ril::prelude::*;

lazy_static::lazy_static! {
    static ref LEGO: Image<Rgb> = Image::open("./assets/lego.png").unwrap();
}

/// helper function for lego to colorize the lego brick
/// with each pixel's color in the image
fn colorize_lego_band(image: Image<L>, value: i32) -> Image<L> {
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
fn resize_to(image: Image<Rgba>, size: u32) -> Image<Rgba> {
    let (w, h) = image.dimensions();
    let (width, height) =
        if w > h {
            (size, ((size as f32 / w as f32) * h as f32).ceil() as u32)
        } else {
            (((size as f32 / h as f32) * w as f32).ceil() as u32, size)
        };

    image.resized(width, height, ResizeAlgorithm::Bilinear)
}

/// builds an image out of lego blocks
pub fn lego(image: Image<Rgba>, SizeOption { size }: SizeOption) -> Image<Rgba> {
    let (mut x, mut y) = (0u32, 0u32);
    let image = resize_to(
        image,
        size.unwrap_or(40) as u32
    );
    let mut base = Image::<Rgba>::new(
        image.width() * 30,
        image.height() * 30,
        Rgba::transparent(),
    );

    for row in image.pixels() {
        for pixel in row {
            if pixel.a != 0 {
                base.paste(x, y, {
                    let (r, g, b) = LEGO.bands();
                    Image::from_bands((
                        colorize_lego_band(r, pixel.r as i32),
                        colorize_lego_band(g, pixel.g as i32),
                        colorize_lego_band(b, pixel.b as i32),
                        Image::new(30, 30, L::new(pixel.a))
                    ))
                });
            }
            x += 30;
        }
        x = 0;
        y += 30;
    }

    base
}

/// WIP not found 404 fallback
pub async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404 not found").into_response()
}