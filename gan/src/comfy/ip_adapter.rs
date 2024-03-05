use anyhow::Context;
use macros::FromValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// IPAdapterApply
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct IPAdapterApply {
    pub weight: f32,
    pub noise: f32,
    //original
    pub weight_type: String,
    pub start_at: f32,
    pub end_at: f32,
    pub unfold_batch: bool,
    pub ipadapter: Value,
    pub clip_vision: Value,
    pub image: Value,
    //
    pub model: Value,
}

/// IPAdapterModelLoader
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct IPAdapterModelLoader {
    pub ipadapter_file: String,
}

/// CLIPVisionLoader
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct CLIPVisionLoader {
    pub clip_name: String,
}
