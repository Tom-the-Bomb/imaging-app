use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::fs::read_dir;
use ril::prelude::*;
use crate::models::*;


/// constant representing the pixel size of each lego brick
const LEGO_SIZE: u32 = 30;

/// constant representing the pixel size of each minecraft block
const MCSIZE: u32 = 20;

lazy_static::lazy_static! {
    static ref LEGO: Image<Rgb> = Image::open("./assets/lego.png")
        .unwrap();

    static ref MC_IMAGES: HashMap<(u8, u8, u8), Image<Rgb>> = {
        let mut map = HashMap::new();

        for file in read_dir("./assets/minecraft").unwrap() {
            let file = file.unwrap();

            if file.file_name()
                .into_string()
                .map_or(false, |s| !s.ends_with(".png"))
            {
                continue;
            }

            if let Ok(block) =
                Image::<Rgb>::open(file.path())
            {
                let single = block.clone()
                    .resized(1, 1, ResizeAlgorithm::Bilinear);
                map.insert(
                    single.pixel(0, 0).as_rgb_tuple(),
                    block.resized(MCSIZE, MCSIZE, ResizeAlgorithm::Bilinear),
                );
            }
        }

        map
    };

    static ref MC_SAMPLE: Vec<(u8, u8, u8)> = MC_IMAGES
        .keys()
        .copied()
        .collect();

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

/// helper function to determine the closest color in the sample to the target pixel
fn get_closest_color(target: (u8, u8, u8)) -> (u8, u8, u8) {
    MC_SAMPLE.iter()
        .min_by_key(|color|
            color.0.abs_diff(target.0) as u32 +
            color.1.abs_diff(target.1) as u32 +
            color.2.abs_diff(target.2) as u32
        )
        .cloned()
        .unwrap()
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
/// of provided `size`, defaulting to 40 blocks
pub fn lego(image: Image<Rgba>, SizeOption { size }: SizeOption) -> Image<Rgba> {
    let (mut x, mut y) = (0u32, 0u32);
    let image = resize_to(
        image,
        size.unwrap_or(40) as u32
    );
    let mut base = Image::<Rgba>::new(
        image.width() * LEGO_SIZE,
        image.height() * LEGO_SIZE,
        Rgba::transparent(),
    );

    for row in image.pixels() {
        for pixel in row {
            if pixel.a > 0 {
                base.paste(x, y, {
                    let (r, g, b) = LEGO.bands();
                    Image::from_bands((
                        colorize_lego_band(r, pixel.r as i32),
                        colorize_lego_band(g, pixel.g as i32),
                        colorize_lego_band(b, pixel.b as i32),
                        Image::new(LEGO_SIZE, LEGO_SIZE, L::new(pixel.a))
                    ))
                });
            }
            x += LEGO_SIZE;
        }
        x = 0;
        y += LEGO_SIZE;
    }

    base
}

/// builds an image out of minecraft blocks
/// of provided `size`, defaulting to 70 blocks
pub fn minecraft(image: Image<Rgba>, SizeOption { size }: SizeOption) -> Image<Rgba> {
    let (mut x, mut y) = (0u32, 0u32);
    let image = resize_to(
        image,
        size.unwrap_or(70) as u32
    );
    let mut base = Image::<Rgba>::new(
        image.width() * MCSIZE,
        image.height() * MCSIZE,
        Rgba::transparent(),
    );

    for row in image.pixels() {
        for pixel in row {
            if pixel.a > 0 {
                base.paste(x, y, {
                    let color = get_closest_color(pixel.as_rgb_tuple());
                    MC_IMAGES.get(&color)
                        .unwrap()
                        .clone()
                        .convert()
                });
            }
            x += MCSIZE;
        }
        x = 0;
        y += MCSIZE;
    }

    base
}

/// WIP not found 404 fallback
pub async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404 not found").into_response()
}