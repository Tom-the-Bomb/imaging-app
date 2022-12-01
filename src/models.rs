use serde::{Serialize, Deserialize};

/// used for `lego` and `mc` endpoints to indicate how many blocks to use for the image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeOption {
    /// size (max number of blocks for a side) for generated image
    pub size: Option<u8>,
}

/// used for `braille` function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrailleOption {
    /// threshold to determine fill or empty
    pub threshold: Option<u8>,
    /// indicates whether to invert pixel values or not
    pub invert: Option<bool>,
    /// size (max length of a side) for generated image
    pub size: Option<u16>,
}

/// used for `ascii` function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsciiOption {
    /// indicates whether to invert pixel values or not
    pub invert: Option<bool>,
    /// size (max length of a side) for generated image
    pub size: Option<u16>,
}

/// an empty struct used in endpoints with no query arguments to accept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoArgs {}