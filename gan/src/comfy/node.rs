use anyhow::Context;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    CtrlnetStack, EfficientLoader, ImagePreprocessor, Inputs, KSampler, LineArtPreprocessor,
    LoadImage, LoraStack, SaveImage, VaeDecode, NODE_CROP_SWITCH_AFTER, NODE_CROP_SWITCH_PRE,
    NODE_PREPROCESSOR_SWITCH_AFTER, NODE_PREPROCESSOR_SWITCH_PRE, NODE_SAVE_IMAGE,
    NODE_UPSAVE_IMAGE, NODE_UPSCALE_SWITCH_AFTER, NODE_UPSCALE_SWITCH_PRE,
};

/// A node in the comfy ui workflow
#[derive(Debug, Serialize)]
pub struct Node {
    /// node inputs
    pub inputs: Inputs,
    /// node type
    pub class_type: String,
    /// node meta
    #[serde(rename = "_meta")]
    pub meta: Meta,
}

use paste::paste;
macro_rules! impl_input_methods {
    ($input_type:ident) => {
        paste! {
            pub fn [<$input_type:snake>](&self) -> &$input_type {
                match &self.inputs {
                    Inputs::$input_type(v) => v,
                    _ => panic!("{} not {}", self.meta.title, stringify!($input_type)),
                }
            }

            pub fn [<$input_type:snake _mut>](&mut self) -> &mut $input_type {
                match &mut self.inputs {
                    Inputs::$input_type(v) => v,
                    _ => panic!("{} not {}", self.meta.title, stringify!($input_type)),
                }
            }
        }
    };
}

impl Node {
    pub fn inputs_mut(&mut self) -> &mut Inputs {
        &mut self.inputs
    }

    impl_input_methods!(CtrlnetStack);
    impl_input_methods!(EfficientLoader);
    impl_input_methods!(ImagePreprocessor);
    impl_input_methods!(SaveImage);
    impl_input_methods!(LoadImage);
    impl_input_methods!(LoraStack);
    impl_input_methods!(KSampler);
    impl_input_methods!(VaeDecode);
    impl_input_methods!(LineArtPreprocessor);
}

/// Node meta
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    /// node title
    pub title: String,
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json = Value::deserialize(deserializer)?;
        let title = json["_meta"]["title"]
            .as_str()
            .context("title")
            .unwrap()
            .to_owned();
        let class_type = json["class_type"]
            .as_str()
            .context("class_type")
            .unwrap()
            .to_owned();
        let meta = Meta { title };
        let inputs = json["inputs"].clone();
        let inputs = match meta.title.as_str() {
            t if t == NODE_PREPROCESSOR_SWITCH_PRE => Inputs::PreprocessorSwitchPre(inputs.into()),
            t if t == NODE_PREPROCESSOR_SWITCH_AFTER => {
                Inputs::PreprocessorSwitchAfter(inputs.into())
            }
            t if t == NODE_UPSCALE_SWITCH_PRE => Inputs::UpscaleSwitchPre(inputs.into()),
            t if t == NODE_UPSCALE_SWITCH_AFTER => Inputs::UpscaleSwitchAfter(inputs.into()),
            t if t == NODE_CROP_SWITCH_PRE => Inputs::CropSwitchPre(inputs.into()),
            t if t == NODE_CROP_SWITCH_AFTER => Inputs::CropSwitchAfter(inputs.into()),
            t if t == NODE_UPSAVE_IMAGE => Inputs::UpscaleSaveImage(inputs.into()),
            t if t == NODE_SAVE_IMAGE => Inputs::SaveImage(inputs.into()),
            t => inputs.try_into().context(t.to_owned()).unwrap(),
        };
        Ok(Self {
            inputs,
            class_type,
            meta,
        })
    }
}
