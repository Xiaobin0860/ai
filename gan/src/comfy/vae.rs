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
