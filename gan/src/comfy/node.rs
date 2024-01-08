use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{AppResult, Inputs};

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
    pub fn disable_all_loras(&mut self) -> AppResult<()> {
        match &mut self.inputs {
            Inputs::CRLoRAStack(lora_stack) => {
                lora_stack.disable_all();
            }
            _ => {
                error!("disable_all_loras: wrong node type! {self:?}");
            }
        }
        todo!()
    }
}

/// Node meta
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    /// node title
    pub title: String,
}
