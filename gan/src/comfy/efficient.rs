use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Efficient loader
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EfficientLoader {
    /// The name of the checkpoint file
    pub ckpt_name: String,
    /// The name of the VAE model
    pub vae_name: String,
    /// The name of the LORA model
    pub lora_name: String,
    /// The positive prompts 支持Tagger版本后连TextConcat0(有Tagger)或连TextString0(无Tagger), 旧版string Value
    pub positive: Value,
    /// The negative prompts
    pub negative: String,
    /// The token normalization method
    pub token_normalization: String,
    /// The prompts weight A1111|comfy|compel|comfy++|down_weight
    pub weight_interpretation: String,
    /// The stack of LORA input [LoRAStack_id, 0]
    pub lora_stack: Option<Value>,
    /// The stack of CN input [ControlNetStack_id, 0]
    pub cnet_stack: Option<Value>,
    /// The strength of the LORA model
    pub lora_model_strength: f32,
    /// The strength of the LORA clip
    pub lora_clip_strength: f32,
    /// The width of an empty latent vector
    pub empty_latent_width: u16,
    /// The height of an empty latent vector
    pub empty_latent_height: u16,
    /// The number of skipped clips during clipping
    pub clip_skip: i8,
    /// The batch size for loading data
    pub batch_size: u8,
}
