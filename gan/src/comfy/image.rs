use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Image preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImagePreprocessor {
    /// The preprocessor used for image processing
    pub preprocessor: String,
    /// The processing resolution
    pub resolution: u32,
    /// The image input [LoadImage_id, 0]
    pub image: Option<Value>,
}

/// Image saver
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaveImage {
    /// The filename prefix
    pub filename_prefix: String,
    /// The image input
    pub images: Option<Value>,
}

/// Image loader
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadImage {
    /// image file name in ComfyUI/input dir
    pub image: String,
    /// upload button
    pub upload: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageScaleToSide {
    pub side_length: usize,
    pub side: String,
    pub upscale_method: String,
    pub crop: String,
    pub image: Option<Value>,
}
