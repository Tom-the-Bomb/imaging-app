//! File containing helper functions used by the
//! individual processing functions for endpoints in `functions.rs`
use photon_rs::PhotonImage;
use ril::prelude::*;
use crate::braille_data::BRAILLE_DATA;

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

/// grayscales a pixel
pub fn grayscale(px: &Rgba) -> u32 {
    (px.r as u32 + px.b as u32 + px.g as u32) / 3
}

/// helper function to quickly write text on a blank image
pub fn draw_text(font: &Font, text: String) -> Image<Rgba> {
    let layout = TextLayout::new()
        .with_wrap(WrapStyle::None)
        .with_position(0, 0)
        .with_basic_text(
            font, text, Rgba::black()
        );
    let mut canvas = Image::<Rgba>::new(
        layout.width(),
        layout.height(),
        Rgba::white()
    );
    canvas.draw(&layout);
    canvas
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

/// converts a RIL [`Image`] to a Photon-rs [`PhotonImage`]
pub fn to_photon(image: Image<Rgba>) -> ril::Result<PhotonImage> {
    let mut buffer = Vec::<u8>::new();
    image.encode(ImageFormat::Png, &mut buffer)?;

    Ok(PhotonImage::new_from_byteslice(buffer))
}

/// converts a Photon-rs [`PhotonImage`] to a RIL [`Image`]
pub fn to_ril(image: PhotonImage) -> Image<Rgba> {
    Image::<Rgba>::from_pixels(
        image.get_width(),
        {
            image.get_raw_pixels()
                .chunks_exact(4)
                .map(|c| Rgba { r: c[0], g: c[1], b: c[2], a: c[3]})
                .collect::<Vec<Rgba>>()
        }
    )
}

/// maps an image pixel value [`Rgba`] to a corresponding braille character
pub fn get_braille_from_px(x: u32, y: u32, image: &Image<Rgba>, invert: bool, threshold: u32) -> Option<String> {
    let mut region = vec![vec!["0", "0"]; 4];
    let (width, height) = image.dimensions();
    for i in x..x + 2 {
        for j in y..y + 4 {
            let mut gray: u32 = 0;

            if !(i >= width || j >= height) {
                gray = {
                    let px = image.get_pixel(i, j)
                        .unwrap();
                    grayscale(px)
                };
            }
            region[(j - y) as usize][(i - x) as usize] =
                (gray < threshold)
                    .then_some(if invert { "0" } else { "1" })
                    .unwrap_or_else(|| if invert { "1" } else { "0" });
        }
    }

    let key = region
        .into_iter()
        .map(|inner| inner.join(""))
        .collect::<Vec<String>>()
        .join(" ");

    BRAILLE_DATA.get(&key)
        .cloned()
}

/// fixes braille string spaces and padding at the end
pub fn fix_braille_spaces(mut matrix: Vec<Vec<String>>, width: usize, height: usize) -> Vec<Vec<String>> {
    for y in 0..height {
        let mut last = width - 1;
        for x in (0..width).rev() {
            if matrix[y][x] != "." {
                break;
            }
            last = x;
        }
        matrix[y] = matrix[y][0..last]
            .to_vec();
    }
    for y in 0..height {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == "." {
                matrix[y][x] = "⢀".to_string();
            }
        }
    }
    matrix
}