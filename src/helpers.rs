//! File containing helper functions used by the
//! individual processing functions for endpoints in `functions.rs`
use photon_rs::PhotonImage;
use ril::prelude::*;
use rand::{thread_rng, Rng};
use crate::braille_data::BRAILLE_DATA;

/// enum for determining type of shape to draw for [`gen_shape_frame`]
#[derive(Debug, Clone, Copy)]
pub enum ShapeMethod {
    Line,
    Ball,
    Square,
}

/// helper function for lego to colorize the lego brick
/// with each pixel's color in the image
pub fn colorize_lego_band(image: Image<L>, value: i32) -> Image<L> {
    image.map_pixels(|p| {
        let p = i32::from(p.value());

        let mut value = if p < 33 {
            value - 100
        } else if p > 233 {
            value + 100
        } else {
            value - 133 + p
        };

        value = value.clamp(0, 255);

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        L::new(value as u8)
    })
}

/// grayscales an RGBA pixel
#[allow(clippy::missing_const_for_fn)]
pub fn grayscale(px: Rgba) -> u32 {
    (u32::from(px.r) + u32::from(px.b) + u32::from(px.g)) / 3
}

/// helper function to quickly write basic text on a blank image with a provided font
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
/// with provided resampling algorithm
pub fn resize_to_alg(image: Image<Rgba>, size: u32, alg: ResizeAlgorithm) -> Image<Rgba> {
    let (w, h) = image.dimensions();

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (width, height) =
        if w > h {
            (size, (
                (f64::from(size) / f64::from(w)) * f64::from(h)
            ).ceil() as u32)
        } else {
            ((
                (f64::from(size) / f64::from(h)) * f64::from(w)
            ).ceil() as u32, size)
        };

    image.resized(width, height, alg)
}

/// same as [`resize_to_alg`]; shorthand with default resampling
pub fn resize_to(image: Image<Rgba>, size: u32) -> Image<Rgba> {
    resize_to_alg(image, size, ResizeAlgorithm::Bilinear)
}

/// converts a RIL [`Image`] to a Photon-rs [`PhotonImage`]
pub fn to_photon(image: &Image<Rgba>) -> ril::Result<PhotonImage> {
    let mut buffer = Vec::<u8>::new();
    image.encode(ImageFormat::Png, &mut buffer)?;

    Ok(PhotonImage::new_from_byteslice(buffer))
}

/// converts a Photon-rs [`PhotonImage`] to a RIL [`Image`]
pub fn to_ril(image: &PhotonImage) -> Image<Rgba> {
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
pub fn get_braille_from_px(x: usize, y: usize, image: &Image<Rgba>, invert: bool, threshold: u32) -> Option<String> {
    let mut region = vec![vec!["0", "0"]; 4];
    let (width, height) = image.dimensions();
    for i in x..x + 2 {
        #[allow(clippy::cast_possible_truncation)]
        for j in y..y + 4 {
            let gray = if i >= width as usize || j >= height as usize
            { 0 } else {
                let px = image.get_pixel(i as u32, j as u32)
                    .unwrap();
                grayscale(*px)
            };

            region[j - y][i - x] =
                if gray < threshold {
                    if invert { "0" } else { "1" }
                } else if invert { "1" } else { "0" };
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
    for row in matrix
        .iter_mut()
        .take(height)
    {
        let mut last = width - 1;
        for x in (0..width).rev() {
            if row[x] != "." {
                break;
            }
            last = x;
        }
        *row = row[0..last]
            .to_vec();
    }

    for row in matrix
        .iter_mut()
        .take(height)
    {
        for item in row.iter_mut() {
            if item == "." {
                *item = "⢀".to_string();
            }
        }
    }
    matrix
}

/// resizes ascii image for proper aspect ratio when rendering the characters
pub fn ascii_resize(image: Image<Rgba>, cols: u32) -> Image<Rgba> {
    let (w, h) = image.dimensions();
    let ratio = f64::from(h) / f64::from(w);

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let rows = (ratio * (f64::from(cols) / 2.0)) as u32;

    image.resized(cols, rows, ResizeAlgorithm::Bicubic)
}

/// generates a shape frame for balls / square / lines
pub fn gen_shape_frame(
    image: &Image<Rgba>,
    method: ShapeMethod,
    size: Option<u8>,
    density: Option<u32>,
) -> Frame<Rgba> {
    let size = u32::from(size.unwrap_or(10));
    let density = density.unwrap_or(10000);
    let (width, height) = image.dimensions();

    let mut rng = thread_rng();
    let mut canvas = Image::<Rgba>::new(
        width, height,
        Rgba::transparent(),
    );
    for _ in 0..density {
        let x = rng.gen_range(1..width);
        let y = rng.gen_range(1..height);
        let (x1, y1, x2, y2) =
            (
                x.saturating_sub(size),
                y.saturating_sub(size),
                x + size,
                y + size,
            );
        let fill = *image.get_pixel(x, y)
            .unwrap();
        match method {
            ShapeMethod::Line =>
                Line::new(
                    (x1, y1),
                    (x2, y2),
                    fill,
                ).draw(&mut canvas),
            ShapeMethod::Ball =>
                Ellipse::from_bounding_box(x1, y1, x2, y2)
                    .with_border(Border::new(Rgba::black(), 1))
                    .with_fill(fill)
                    .draw(&mut canvas),
            ShapeMethod::Square =>
                Rectangle::from_bounding_box(x1, y1, x2, y2)
                    .with_fill(fill)
                    .draw(&mut canvas),
        }
    }
    Frame::from_image(canvas)
}