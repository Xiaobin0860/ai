use serde::{Deserialize, Serialize};

use crate::{Inputs, LoadImage};

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

impl Node {
    pub fn get_inputs_mut(&mut self) -> &mut Inputs {
        &mut self.inputs
    }

    pub fn load_image(&self) -> &LoadImage {
        match &self.inputs {
            Inputs::LoadImage(v) => v,
            _ => {
                panic!("{} not {}", self.class_type, "LoadImage");
            }
        }
    }

    pub fn load_image_mut(&mut self) -> &mut LoadImage {
        match &mut self.inputs {
            Inputs::LoadImage(v) => v,
            _ => {
                panic!("{} not {}", self.class_type, "LoadImage");
            }
        }
    }
}

/// Node meta
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    /// node title
    pub title: String,
}
