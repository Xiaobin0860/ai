use anyhow::Context;
use macros::FromValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 文生图|图生图开关 "CR Img2Img Process Switch"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxtimgSwitch {
    /// txt2img|img2img
    #[serde(rename = "Input")]
    pub input: String,
    // 应该不会自己创建, 解析要求都有
    pub txt2img: Value,
    pub img2img: Value,
}

/// PreprocessorSwitchPre 预处理开关 ImpactInversedSwitch
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct PreprocessorSwitchPre {
    /// select [1,11]
    pub select: u8,
    pub input: Value,
}
/// PreprocessorSwitchAfter 预处理开关 ImpactSwitch
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct PreprocessorSwitchAfter {
    /// select [1,11]
    pub select: u8,
    /// true
    pub sel_mode: bool,
    //
    pub input1: Value,
    pub input2: Value,
    pub input3: Value,
    pub input4: Value,
    pub input5: Value,
    pub input6: Value,
    pub input7: Value,
    pub input8: Value,
    pub input9: Value,
    pub input10: Value,
}
