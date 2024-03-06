use serde::{Deserialize, Serialize};

use crate::{
    CLIPVisionLoader, CannyEdgePreprocessor, CropImage, CtrlnetStack, EfficientLoader, EmptyImage,
    EmptyLatent, IPAdapterApply, IPAdapterModelLoader, ImageFilter, ImagePreprocessor, ImageRembg,
    ImageSave, ImageScaleSide, Inputs, KSampler, LineArtPreprocessor, LineartStandardPreprocessor,
    LoadImage, LoraStack, LoraStacker, RepeatLatent, SaveImage, Tagger, TextConcat, TextString,
    TilePreprocessor, VaeDecode,
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

    #[serde(skip)]
    pub id: String,
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
    impl_input_methods!(ImageSave);
    impl_input_methods!(CropImage);
    impl_input_methods!(SaveImage);
    impl_input_methods!(RepeatLatent);
    impl_input_methods!(EmptyLatent);
    impl_input_methods!(LoadImage);
    impl_input_methods!(ImageFilter);
    impl_input_methods!(EmptyImage);
    impl_input_methods!(ImageRembg);
    impl_input_methods!(ImageScaleSide);
    impl_input_methods!(LoraStack);
    impl_input_methods!(LoraStacker);
    impl_input_methods!(KSampler);
    impl_input_methods!(VaeDecode);
    impl_input_methods!(LineArtPreprocessor);
    impl_input_methods!(LineartStandardPreprocessor);
    impl_input_methods!(TilePreprocessor);
    impl_input_methods!(CannyEdgePreprocessor);
    impl_input_methods!(IPAdapterApply);
    impl_input_methods!(IPAdapterModelLoader);
    impl_input_methods!(CLIPVisionLoader);
    impl_input_methods!(TextString);
    impl_input_methods!(TextConcat);
    impl_input_methods!(Tagger);
}

/// Node meta
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    /// node title
    pub title: String,
}
