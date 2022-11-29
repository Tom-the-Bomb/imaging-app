use serde::{Serialize, Deserialize};

/// used for `lego` and `mc` endpoints to indicate how many blocks to use for the image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeOption {
    pub size: Option<u8>,
}

/// used for `braille`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrailleOption {
    pub threshold: Option<u8>,
    pub invert: Option<bool>,
    pub size: Option<u16>,
}

/// an empty struct used in endpoints with no query arguments to accept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoArgs {}