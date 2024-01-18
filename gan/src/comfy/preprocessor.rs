use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Canny edge preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CannyEdgePreprocessor {
    pub low_threshold: u8,
    pub high_threshold: u8,
    pub resolution: u16,
    pub image: Value,
}

/// Openpose preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenposePreprocessor {
    pub detect_hand: String,
    pub detect_body: String,
    pub detect_face: String,
    pub resolution: u16,
    pub image: Value,
}

/// Line art preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineArtPreprocessor {
    pub coarse: String,
    pub resolution: u16,
    pub image: Value,
}

/// Tile preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TilePreprocessor {
    //pyrUp_iters {"default": 3, "min": 1, "max": 10, "step": 1}
    #[serde(rename = "pyrUp_iters")]
    pub pyrup_iters: u8,
    pub resolution: u16,
    pub image: Option<Value>,
}

/// HED preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HEDPreprocessor {
    pub safe: String,
    pub resolution: u16,
    pub image: Option<Value>,
}

/// LeReS depth map preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeReSDepthMapPreprocessor {
    // rm_nearest {"default": 0.0, "min": 0.0, "max": 100, "step": 0.1}
    pub rm_nearest: f32,
    // rm_background {"default": 0.0, "min": 0.0, "max": 100, "step": 0.1}
    pub rm_background: f32,
    pub boost: String,
    pub resolution: u16,
    pub image: Option<Value>,
}

/// MiDaS depth map preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MiDaSDepthMapPreprocessor {
    // a {"default": np.pi * 2.0, "min": 0.0, "max": np.pi * 5.0, "step": 0.05}
    pub a: f32,
    // bg_threshold {"default": 0.1, "min": 0, "max": 1, "step": 0.05}
    pub bg_threshold: f32,
    pub resolution: u16,
    pub image: Option<Value>,
}

/// Zoe depth map preprocessor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoeDepthMapPreprocessor {
    pub resolution: u16,
    pub image: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineartStandardPreprocessor {
    // guassian_sigma {"default": 6.0, "min": 0.0, "max": 100.0}
    pub guassian_sigma: f32,
    // intensity_threshold {"default": 8, "min": 0, "max": 16}
    pub intensity_threshold: u8,
    pub resolution: u16,
    pub image: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLSDPreprocessor {
    // score_threshold {"default": 0.1, "min": 0.01, "max": 2.0, "step": 0.01}
    pub score_threshold: f32,
    // dist_threshold {"default": 0.1, "min": 0.01, "max": 20.0, "step": 0.01}
    pub dist_threshold: f32,
    pub resolution: u16,
    pub image: Option<Value>,
}
