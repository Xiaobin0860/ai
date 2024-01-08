use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Inputs, Node};

/// VAE decoder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaeDecode {
    /// samples input
    pub samples: Vec<Value>,
    /// vae input
    pub vae: Vec<Value>,
}

impl From<&Node> for VaeDecode {
    fn from(value: &Node) -> Self {
        match &value.inputs {
            Inputs::VaeDecode(v) => v.clone(),
            _ => panic!("VaeDecode"),
        }
    }
}
