use serde::{Deserialize, Serialize};
use serde_json::Value;

/// VAE decoder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaeDecode {
    /// samples input
    pub samples: Value,
    /// vae input
    pub vae: Value,
}

/// VAE encoder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VAEEncode {
    /// samples input
    pub pixels: Value,
    /// vae input
    pub vae: Value,
}
