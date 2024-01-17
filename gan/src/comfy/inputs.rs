use anyhow::Context;
use macros::FromNode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    CannyEdgePreprocessor, CropImage, CropSwitchAfter, CropSwitchPre, CtrlnetStack,
    EfficientLoader, EmptyLatent, HEDPreprocessor, ImagePreprocessor, ImageScaleToSide, KSampler,
    LeReSDepthMapPreprocessor, LineArtPreprocessor, LineartStandardPreprocessor, LoadImage,
    LoraStack, MLSDPreprocessor, MiDaSDepthMapPreprocessor, OpenposePreprocessor,
    PreprocessorSwitchAfter, PreprocessorSwitchPre, SaveImage, TilePreprocessor, TxtimgSwitch,
    UpscaleImage, UpscaleSaveImage, UpscaleSwitchAfter, UpscaleSwitchPre, VaeDecode, VaeEncode,
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
    /// ImagePreprocessor
    ImagePreprocessor(ImagePreprocessor),
    /// Efficient Loader
    EfficientLoader(EfficientLoader),
    /// KSampler
    KSampler(KSampler),
    /// VAE Decode
    VaeDecode(VaeDecode),
    VaeEncode(VaeEncode),
    /// Save Image
    SaveImage(SaveImage),
    ImageScaleToSide(ImageScaleToSide),
    UpscaleImage(UpscaleImage),
    UpscaleSaveImage(UpscaleSaveImage),
    CropImage(CropImage),

    EmptyLatent(EmptyLatent),

    /// Preprocessor
    CannyEdgePreprocessor(CannyEdgePreprocessor),
    OpenposePreprocessor(OpenposePreprocessor),
    LineArtPreprocessor(LineArtPreprocessor),
    TilePreprocessor(TilePreprocessor),
    HEDPreprocessor(HEDPreprocessor),
    LeReSDepthMapPreprocessor(LeReSDepthMapPreprocessor),
    MiDaSDepthMapPreprocessor(MiDaSDepthMapPreprocessor),
    LineartStandardPreprocessor(LineartStandardPreprocessor),
    MLSDPreprocessor(MLSDPreprocessor),

    /// Switch
    TxtimgSwitch(TxtimgSwitch),
    UpscaleSwitchPre(UpscaleSwitchPre),
    UpscaleSwitchAfter(UpscaleSwitchAfter),
    PreprocessorSwitchPre(PreprocessorSwitchPre),
    PreprocessorSwitchAfter(PreprocessorSwitchAfter),
    CropSwitchPre(CropSwitchPre),
    CropSwitchAfter(CropSwitchAfter),
}

impl TryFrom<Value> for Inputs {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).context("Inputs")
    }
}
