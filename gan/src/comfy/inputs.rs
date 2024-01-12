use macros::FromNode;
use serde::{Deserialize, Serialize};

use crate::{
    CannyEdgePreprocessor, CtrlnetStack, EfficientLoader, HEDPreprocessor, ImagePreprocessor,
    KSampler, LeReSDepthMapPreprocessor, LineArtPreprocessor, LineartStandardPreprocessor,
    LoadImage, LoraStack, MLSDPreprocessor, MiDaSDepthMapPreprocessor, OpenposePreprocessor,
    SaveImage, TilePreprocessor, VaeDecode,
};

// TODO: 准确类型可能需要自已实现根据class_type来判断, 直接ComfyUI api json解析丢失类型信息
/// Node inputs
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
#[derive(FromNode)]
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
    CannyEdgePreprocessor(CannyEdgePreprocessor),
    OpenposePreprocessor(OpenposePreprocessor),
    LineArtPreprocessor(LineArtPreprocessor),
    TilePreprocessor(TilePreprocessor),
    HEDPreprocessor(HEDPreprocessor),
    LeReSDepthMapPreprocessor(LeReSDepthMapPreprocessor),
    MiDaSDepthMapPreprocessor(MiDaSDepthMapPreprocessor),
    LineartStandardPreprocessor(LineartStandardPreprocessor),
    MLSDPreprocessor(MLSDPreprocessor),
}
