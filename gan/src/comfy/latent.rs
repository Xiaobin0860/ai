use serde::{Deserialize, Serialize};

/// EmptyLatentImage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyLatent {
    pub width: u16,
    pub height: u16,
    pub batch_size: u8,
}
