use serde::{Deserialize, Serialize};
use serde_json::Value;

/// EmptyLatentImage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyLatent {
    pub width: u16,
    pub height: u16,
    pub batch_size: u8,
}

/// RepeatLatentBatch
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepeatLatent {
    pub amount: u8,
    pub samples: Value,
}
