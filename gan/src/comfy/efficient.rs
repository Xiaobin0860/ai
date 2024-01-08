use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Inputs, Node};

/// Efficient loader
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EfficientLoader {
    /// The name of the checkpoint file
    pub ckpt_name: String,
    /// The name of the VAE model
    pub vae_name: String,
    /// The name of the LORA model
    pub lora_name: String,
    /// The positive prompts
    pub positive: String,
    /// The negative prompts
    pub negative: String,
    /// The token normalization method
    pub token_normalization: String,
    /// The prompts weight A1111|comfy|compel|comfy++|down_weight
    pub weight_interpretation: String,
    /// The stack of LORA input [LoRAStack_id, 0]
    pub lora_stack: Vec<Value>,
    /// The stack of CN input [ControlNetStack_id, 0]
    pub cnet_stack: Vec<Value>,
    /// The strength of the LORA model
    pub lora_model_strength: f32,
    /// The strength of the LORA clip
    pub lora_clip_strength: f32,
    /// The width of an empty latent vector
    pub empty_latent_width: u16,
    /// The height of an empty latent vector
    pub empty_latent_height: u16,
    /// The number of skipped clips during clipping
    pub clip_skip: i16,
    /// The batch size for loading data
    pub batch_size: u16,
}

impl From<&Node> for EfficientLoader {
    fn from(value: &Node) -> Self {
        match &value.inputs {
            Inputs::EfficientLoader(v) => v.clone(),
            _ => panic!("EfficientLoader"),
        }
    }
}
