use serde::{Deserialize, Serialize};

use crate::{
    CtrlnetStack, EfficientLoader, ImagePreprocessor, KSampler, LoadImage, LoraStack, SaveImage,
    VaeDecode,
};

// TODO: 应该可以用宏来自动生成置换和访问代码
/// Node inputs
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Inputs {
    /// Load Image
    LoadImage(LoadImage),
    /// CR LoRA Stack
    LoraStack(LoraStack),
    /// CR Multi-ControlNet Stack
    CtrlnetStack(CtrlnetStack),
    /// AIO Aux Preprocessor
    ImagePreprocessor(ImagePreprocessor),
    /// Efficient Loader
    EfficientLoader(EfficientLoader),
    /// KSampler
    KSampler(KSampler),
    /// VAE Decode
    VaeDecode(VaeDecode),
    /// Save Image
    SaveImage(SaveImage),
}
