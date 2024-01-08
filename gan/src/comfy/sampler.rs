use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Inputs, Node};

/// K-sampler
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KSampler {
    pub seed: i64,
    pub sampler_name: String,
    pub scheduler: String,
    pub model: Vec<Value>,
    pub positive: Vec<Value>,
    pub negative: Vec<Value>,
    pub latent_image: Vec<Value>,
    /// prompt strength [1, 30]
    pub cfg: f32,
    /// denoise [0, 1]
    pub denoise: f32,
    /// generate steps [1, 30]
    pub steps: u8,
}

impl From<&Node> for KSampler {
    fn from(value: &Node) -> Self {
        match &value.inputs {
            Inputs::KSampler(v) => v.clone(),
            _ => panic!("KSampler"),
        }
    }
}
