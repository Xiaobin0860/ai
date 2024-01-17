use anyhow::Context;
use macros::FromValue;
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

/// Image saver, SaveImage
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct SaveImage {
    /// The filename prefix
    pub filename_prefix: String,
    /// The image input
    pub images: Option<Value>,
}

/// UpscaleImage saver, SaveImage
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct UpscaleSaveImage {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpscaleImage {
    pub upscale_model: String,
    /// rescale|resize
    pub mode: String,
    pub rescale_factor: f32,
    pub resize_width: u16,
    /// bilinear|nearest|bicubic|lanczos
    pub resampling_method: String,
    /// true|false
    pub supersample: String,
    pub rounding_modulus: u16,
    pub image: Option<Value>,
}

/// CropImage "Crop Image TargetSize (JPS)"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CropImage {
    pub target_w: u16,
    pub target_h: u16,
    /// center
    pub crop_position: String,
    pub offset: i32,
    /// bilinear
    pub interpolation: String,
    pub image: Option<Value>,
}
