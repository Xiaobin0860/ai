use serde::{Deserialize, Serialize};

use crate::{
    ControlNetStack, EfficientLoader, ImagePreprocessor, KSampler, LoRAStack, LoadImage, SaveImage,
    VAEDecode,
};

/// Node inputs
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Inputs {
    /// Load Image
    LoadImage(LoadImage),
    /// CR LoRA Stack
    CRLoRAStack(LoRAStack),
    /// CR Multi-ControlNet Stack
    CRControlNetStack(ControlNetStack),
    /// AIO Aux Preprocessor
    ImagePreprocessor(ImagePreprocessor),
    /// Efficient Loader
    EfficientLoader(EfficientLoader),
    /// KSampler
    KSampler(KSampler),
    /// VAE Decode
    VAEDecode(VAEDecode),
    /// Save Image
    SaveImage(SaveImage),
}
