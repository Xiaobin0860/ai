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
    pub resolution: u16,
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

/// Image Save
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct ImageSave {
    pub output_path: String,
    pub filename_prefix: String,
    pub filename_delimiter: String,
    pub filename_number_padding: u8,
    pub filename_number_start: String,
    pub extension: String,
    pub quality: u8,
    pub lossless_webp: String,
    pub overwrite_mode: String,
    pub show_history: String,
    pub show_history_by_prefix: String,
    pub embed_workflow: String,
    pub show_previews: String,
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

/// ImageScaleSide "Image scale to side"
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct ImageScaleSide {
    pub side_length: u16,
    /// side Longest|Width|Height
    pub side: String,
    /// area|nearest-exact|bilinear
    pub upscale_method: String,
    /// disabled|center
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
    #[serde(default)]
    pub sharpening: f32,
    pub image: Option<Value>,
}

/// ImageFilter "Image Filter Adjustments"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageFilter {
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub sharpness: f32,
    /// [0, 16]
    pub blur: u8,
    pub gaussian_blur: f32,
    pub edge_enhance: f32,
    pub detail_enhance: String,
    pub image: Option<Value>,
}

/// EmptyImage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyImage {
    pub width: u16,
    pub height: u16,
    pub batch_size: u8,
    pub color: u32,
}

/// ImageRembg "Image Remove Background (rembg)"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageRembg {
    pub model_name: String,
    pub image: Option<Value>,
}

/// Tagger "WD14Tagger|pysssss"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tagger {
    /// wd-v1-4-convnextv2-tagger-v2|wd-v1-4-moat-tagger-v2
    pub model: String,
    pub threshold: f32,
    pub character_threshold: f32,
    pub replace_underscore: bool,
    pub trailing_comma: bool,
    pub exclude_tags: String,
    /// input image ImageFilter
    pub image: Option<Value>,
}
