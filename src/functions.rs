//! File containing all processing functions for indivdual endpoints

use std::{path::Path, collections::HashMap};
use rand::{thread_rng, Rng};
use std::fs::read_dir;
use photon_rs::effects;
use ril::prelude::*;

#[allow(clippy::wildcard_imports)]
use crate::{
    helpers::*,
    models::*,
};


/// constant representing the pixel size of each lego brick
const LEGO_SIZE: u32 = 30;

/// constant representing the pixel size of each minecraft block
const MCSIZE: u32 = 20;

/// shortcut typealias for return type of all functions
type R = ril::Result<Image<Rgba>>;
/// shortcut typealias but for for animated results
type RGif = ril::Result<ImageSequence<Rgba>>;

lazy_static::lazy_static! {
    /// gray lego brick asset
    static ref LEGO: Image<Rgb> = Image::open("./assets/lego.png")
        .unwrap();
    /// unicode font used for `braille` (supports braille glyphs)
    static ref UNICODE_FONT: Font = Font::open("./assets/unicode.ttf", 30.0)
        .unwrap();
    /// monospace font (consolas) used for `ascii` (equal in spacing)
    static ref MONOSPACE_FONT: Font = Font::open("./assets/monospace.ttf", 30.0)
        .unwrap();
    /// "programming / code" font used for `matrix`
    static ref CODE_FONT: Font = Font::open("./assets/monaco-linux.ttf", 30.0)
        .unwrap();
    /// constant storing all the characters used in the `ascii` function
    static ref ASCII_CHARS: Vec<&'static str> = vec![
        "@", "#", "S", "%", "?", "*", "+", ";", ":", ",", ".", " ",
    ];
    static ref CHAR_SAMPLE: Vec<&'static str> = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g",
        "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",
        "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "!", r#"""#, "#", "$", "%", "&",
        "'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@", "[", r"\",
        "]", "^", "_", "`", "{", "|", "}", "~", " ", "\t", "\n", "\r", "\x0b", "\x0c",
    ];

    /// mapping containing all minecraft assets stored as (color: image) pairs
    static ref MC_IMAGES: HashMap<(u8, u8, u8, u8), Image<Rgba>> = {
        let mut failed = 0;
        let mut map = HashMap::new();

        for file in read_dir("./assets/minecraft").unwrap() {
            let file = file.unwrap();

            if file.file_name()
                .into_string()
                .map_or(false, |s| !Path::new(&s)
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("png"))
                )
            {
                continue;
            }

            #[allow(clippy::option_if_let_else)]
            if let Ok(block) =
                Image::<Rgba>::open(file.path())
            {
                let single = block.clone()
                    .resized(1, 1, ResizeAlgorithm::Bilinear);
                map.insert(
                    single.pixel(0, 0).as_rgba_tuple(),
                    block.resized(MCSIZE, MCSIZE, ResizeAlgorithm::Bilinear),
                );
            } else {
                failed += 1;
            }
        }

        println!("Loaded {} minecraft blocks", map.len());
        println!("Failed to load {failed} images");
        map
    };

    /// a collection of all colors (palette) of the minecraft assets
    static ref MC_SAMPLE: Vec<(u8, u8, u8, u8)> = MC_IMAGES
        .keys()
        .copied()
        .collect();

}


/// builds an image out of lego blocks
/// of provided `size`, defaulting to 40 blocks
#[allow(clippy::unnecessary_wraps, clippy::many_single_char_names)]
pub fn lego(image: Image<Rgba>, SizeOption { size }: SizeOption) -> R {
    let (mut x, mut y) = (0u32, 0u32);
    let image = resize_to(
        image,
        u32::from(size.unwrap_or(40))
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
                    &Image::from_bands((
                        colorize_lego_band(r, i32::from(pixel.r)),
                        colorize_lego_band(g, i32::from(pixel.g)),
                        colorize_lego_band(b, i32::from(pixel.b)),
                        Image::new(LEGO_SIZE, LEGO_SIZE, L::new(pixel.a))
                    ))
                });
            }
            x += LEGO_SIZE;
        }
        x = 0;
        y += LEGO_SIZE;
    }

    Ok(base)
}

/// builds an image out of minecraft blocks
/// of provided `size`, defaulting to 70 blocks
#[allow(clippy::unnecessary_wraps)]
pub fn minecraft(image: Image<Rgba>, SizeOption { size }: SizeOption) -> R {
    let (mut x, mut y) = (0u32, 0u32);
    let image = resize_to(
        image,
        u32::from(size.unwrap_or(70))
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
                    let color = MC_SAMPLE.iter()
                        .min_by_key(|color|
                            u32::from(color.0.abs_diff(pixel.r)) +
                            u32::from(color.1.abs_diff(pixel.g)) +
                            u32::from(color.2.abs_diff(pixel.b)) +
                            u32::from(color.3.abs_diff(pixel.a))
                        )
                        .copied()
                        .unwrap();

                    &MC_IMAGES.get(&color)
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

    Ok(base)
}

/// paints out an image
pub fn paint(image: Image<Rgba>, PaintOption { radius, intensity }: PaintOption) -> R {
    let image = resize_to(
        image, 360,
    );
    let mut img = to_photon(&image)?;

    let radius = radius.unwrap_or(5);
    let intensity = intensity.unwrap_or(60.0);
    effects::oil(&mut img, radius, intensity);

    let image = to_ril(&img);
    Ok(image)
}

/// frosted glass effect?
pub fn frost(image: Image<Rgba>, _: NoArgs) -> R {
    let image = resize_to(
        image, 360,
    );
    let mut img = to_photon(&image)?;
    effects::frosted_glass(&mut img);

    let image = to_ril(&img);
    Ok(image)
}

/// emboss effect
pub fn emboss(image: Image<Rgba>, _: NoArgs) -> R {
    let image = resize_to(
        image, 360,
    );
    let mut img = to_photon(&image)?;
    photon_rs::conv::emboss(&mut img);

    let image = to_ril(&img);
    Ok(image)
}

/// "edge" effect
pub fn edge(image: Image<Rgba>, _: NoArgs) -> R {
    let image = resize_to(
        image, 360,
    );
    let mut img = to_photon(&image)?;
    photon_rs::conv::edge_one(&mut img);
    photon_rs::conv::sharpen(&mut img);

    let image = to_ril(&img);
    Ok(image)
}

/// black / white pixels
#[allow(clippy::unnecessary_wraps)]
pub fn black_white(image: Image<Rgba>, SmoothOption { smooth }: SmoothOption) -> R {
    let image = resize_to(
        image, 80,
    );
    let image = resize_to_alg(
        image, 400,
        if smooth.unwrap_or(false) { ResizeAlgorithm::Bilinear }
        else { ResizeAlgorithm::Nearest },
    )
        .convert::<BitPixel>()
        .convert::<Rgba>();

    Ok(image)
}

/// rotates the hue (hsv) value of the image 360deg
#[allow(clippy::unnecessary_wraps)]
pub fn hue_rotate(image: Image<Rgba>, _: NoArgs) -> RGif {
    let image = resize_to(
        image, 360,
    );
    let mut sequence =
        ImageSequence::<Rgba>::new();

    for deg in (0..360).step_by(10) {
        let clone = image.clone()
            .hue_rotated(deg);
        sequence.push_frame(Frame::from_image(clone));
    }

    Ok(sequence)
}

/// builds an image out of braille characters
#[allow(
    clippy::cast_possible_truncation,
    clippy::unnecessary_wraps,
    clippy::cast_sign_loss,
)]
pub fn braille(image: Image<Rgba>, BrailleOption { size, threshold, invert }: BrailleOption) -> R {
    let image = resize_to(
        image,
        u32::from(size.unwrap_or(130))
    );
    let w = (f64::from(image.width()) / 2.0).ceil() as usize;
    let h = (f64::from(image.height()) / 4.0).ceil() as usize;
    let mut mat = vec![vec![" ".to_string(); w]; h];

    for x in 0..w {
        for (y, row) in mat
            .iter_mut()
            .enumerate()
            .take(h)
        {
            row[x] = get_braille_from_px(
                x * 2,
                y * 4,
                &image,
                invert.unwrap_or(false),
                u32::from(threshold.unwrap_or(90)),
            ).unwrap_or_else(|| ".".to_string());
        }
    }
    mat = fix_braille_spaces(mat, w, h);
    let text = mat
        .into_iter()
        .map(|inner| inner.join(""))
        .collect::<Vec<String>>()
        .join("\n");

    let canvas = draw_text(&UNICODE_FONT, text);
    Ok(canvas)
}

/// builds an image out of ascii punctuation characters
#[allow(clippy::unnecessary_wraps)]
pub fn ascii(image: Image<Rgba>, AsciiOption { size, invert }: AsciiOption) -> R {
    let mut image = ascii_resize(
        image,
        u32::from(size.unwrap_or(130))
    );
    if invert.unwrap_or(false) {
        image.invert();
    }
    let image = image.convert::<L>();
    let mut text = String::new();
    for row in image.pixels() {
        for pixel in row {
            text.push_str(ASCII_CHARS[pixel.value() as usize / 25]);
        }
        text.push('\n');
    }
    let canvas = draw_text(&MONOSPACE_FONT, text);
    Ok(canvas)
}

/// builds an image out of ascii punctuation characters
#[allow(clippy::unnecessary_wraps)]
pub fn matrix(image: Image<Rgba>, MatrixOption { size, num_only }: MatrixOption) -> ril::Result<ImageSequence<Rgb>> {
    let image = resize_to(
        image,
        u32::from(size.unwrap_or(80))
    );
    let mut sequence = ImageSequence::<Rgb>::new();

    for _ in 0..5 {
        let (mut x, mut y) = (0u32, 0u32);
        let mut canvas = Image::<Rgb>::new(
            image.width() * 30,
            image.height() * 30,
            Rgb::black(),
        );
        let mut rng = thread_rng();
        for row in image.pixels() {
            for px in row {
                if px.a > 0 {
                    let chr = if num_only.unwrap_or(false) {
                        rng.gen_range(0..=9)
                            .to_string()
                    } else {
                        CHAR_SAMPLE[rng.gen_range(0..CHAR_SAMPLE.len())]
                            .to_string()
                    };
                    let layout = TextLayout::new()
                        .with_wrap(WrapStyle::None)
                        .with_position(x, y)
                        .with_basic_text(&CODE_FONT, chr, px.into_rgb());
                    canvas.draw(&layout);
                }
                x += 30;
            }
            x = 0;
            y += 30;
        }
        sequence.push_frame(Frame::from_image(canvas));
    }
    Ok(sequence)
}

/// builds a shape out of diagonal lines
#[allow(clippy::unnecessary_wraps)]
pub fn lines(image: Image<Rgba>, ShapesOption { block, density, gif }: ShapesOption) -> RGif {
    let image = resize_to(
        image, 360,
    );
    let mut sequence = ImageSequence::<Rgba>::new();
    let t = if gif.unwrap_or(true)
        { 3 } else { 1 };

    for _ in 0..t {
        sequence.push_frame(
            gen_shape_frame(&image, ShapeMethod::Line, block, density)
        );
    }
    Ok(sequence)
}

/// builds a shape out of circles
#[allow(clippy::unnecessary_wraps)]
pub fn balls(image: Image<Rgba>, ShapesOption { block, density, gif }: ShapesOption) -> RGif {
    let image = resize_to(
        image, 360,
    );
    let mut sequence = ImageSequence::<Rgba>::new();
    let t = if gif.unwrap_or(true)
        { 3 } else { 1 };

    for _ in 0..t {
        sequence.push_frame(
            gen_shape_frame(&image, ShapeMethod::Ball, block, density)
        );
    }
    Ok(sequence)
}

/// builds a shape out of squares
#[allow(clippy::unnecessary_wraps)]
pub fn squares(image: Image<Rgba>, ShapesOption { block, density, gif }: ShapesOption) -> RGif {
    let image = resize_to(
        image, 360,
    );
    let mut sequence = ImageSequence::<Rgba>::new();
    let t = if gif.unwrap_or(true)
        { 3 } else { 1 };

    for _ in 0..t {
        sequence.push_frame(
            gen_shape_frame(&image, ShapeMethod::Square, block, density)
        );
    }
    Ok(sequence)
}