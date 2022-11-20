use serde::{Serialize, Deserialize};

/// used for `lego` and `mc` endpoints to indicate how many blocks to use for the image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeOption {
    pub size: Option<u8>,
}

/// used for `paint`; determines if the paint animation should be applied or not
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsGif {
    pub gif: Option<bool>,
}

/// an empty struct used in endpoints with no query arguments to accept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoArgs {}