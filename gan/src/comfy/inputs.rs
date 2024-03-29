use anyhow::Context;
use macros::FromNode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    CLIPVisionLoader, CannyEdgePreprocessor, CropImage, CtrlnetStack, EfficientLoader, EmptyImage,
    EmptyLatent, HEDPreprocessor, IPAdapterApply, IPAdapterModelLoader, ImageFilter,
    ImagePreprocessor, ImageRembg, ImageSave, ImageScaleSide, KSampler, LeReSDepthMapPreprocessor,
    LineArtPreprocessor, LineartStandardPreprocessor, LoadImage, LoraStack, LoraStacker,
    MLSDPreprocessor, MiDaSDepthMapPreprocessor, OpenposePreprocessor, PreprocessorSwitchAfter,
    PreprocessorSwitchPre, RepeatLatent, SaveImage, Tagger, TextConcat, TextString,
    TilePreprocessor, TxtimgSwitch, UpscaleImage, VaeDecode, VaeEncode,
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
    LoraStacker(LoraStacker),
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
    ImageSave(ImageSave),
    SaveImage(SaveImage),
    ImageScaleSide(ImageScaleSide),
    UpscaleImage(UpscaleImage),
    CropImage(CropImage),
    ImageFilter(ImageFilter),
    EmptyImage(EmptyImage),
    ImageRembg(ImageRembg),

    EmptyLatent(EmptyLatent),
    RepeatLatent(RepeatLatent),

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

    /// IPAdapter
    IPAdapterApply(IPAdapterApply),
    IPAdapterModelLoader(IPAdapterModelLoader),
    CLIPVisionLoader(CLIPVisionLoader),

    /// Switch
    TxtimgSwitch(TxtimgSwitch),
    PreprocessorSwitchPre(PreprocessorSwitchPre),
    PreprocessorSwitchAfter(PreprocessorSwitchAfter),

    /// Text
    TextString(TextString),
    TextConcat(TextConcat),
    Tagger(Tagger),
}

impl TryFrom<Value> for Inputs {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).context("Inputs")
    }
}
