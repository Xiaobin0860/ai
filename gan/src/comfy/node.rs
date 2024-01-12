use serde::{Deserialize, Serialize};

use crate::{
    CtrlnetStack, EfficientLoader, ImagePreprocessor, Inputs, KSampler, LineArtPreprocessor,
    LoadImage, LoraStack, SaveImage, VaeDecode,
};

/// A node in the comfy ui workflow
#[derive(Debug, Serialize, Deserialize)]
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
                    _ => panic!("{} not {}", self.class_type, stringify!($input_type)),
                }
            }

            pub fn [<$input_type:snake _mut>](&mut self) -> &mut $input_type {
                match &mut self.inputs {
                    Inputs::$input_type(v) => v,
                    _ => panic!("{} not {}", self.class_type, stringify!($input_type)),
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
