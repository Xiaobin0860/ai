use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CNT_CANNY: &str = "Canny";
pub const CNT_DEPTH: &str = "Depth";
pub const CNT_NORMAL_MAP: &str = "NormalMap";
pub const CNT_OPENPOSE: &str = "OpenPose";
pub const CNT_LINEART: &str = "Lineart";
pub const CNT_ANIME_LINEART: &str = "AnimeLineart";
pub const CNT_SOFT_EDGE: &str = "SoftEdge";
pub const CNT_SEGMENTATION: &str = "Segmentation";
pub const CNT_TILE: &str = "Tile";

pub enum IdxControlNet {
    ControlNet1,
    ControlNet2,
    ControlNet3,
}

/// ControlNet stack
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtrlnetStack {
    /// The switch 1
    pub switch_1: String,
    /// The control net 1 model name
    pub controlnet_1: String,
    /// The strength of control net 1
    pub controlnet_strength_1: f32,
    /// The start percent of control net 1
    pub start_percent_1: f32,
    /// The end percent of control net 1
    pub end_percent_1: f32,
    /// The switch 2
    pub switch_2: String,
    /// The control net 2 model name
    pub controlnet_2: String,
    /// The strength of control net 2
    pub controlnet_strength_2: f32,
    /// The start percent of control net 2
    pub start_percent_2: f32,
    /// The end percent of control net 2
    pub end_percent_2: f32,
    /// The switch 3
    pub switch_3: String,
    /// The control net 3 model name
    pub controlnet_3: String,
    /// The strength of control net 3
    pub controlnet_strength_3: f32,
    /// The start percent of control net 3
    pub start_percent_3: f32,
    /// The end percent of control net 3
    pub end_percent_3: f32,
    /// control net 1 image input
    pub image_1: Option<Value>,
    /// control net 2 image input
    pub image_2: Option<Value>,
    /// control net 3 image input
    pub image_3: Option<Value>,
}

pub struct CnCfg {
    pub model: String,
    pub preprocessor: String,
    pub weight: f32,
    pub start: f32,
    pub end: f32,
}
impl Default for CnCfg {
    fn default() -> Self {
        Self {
            model: "None".into(),
            preprocessor: "None".into(),
            weight: 1.0,
            start: 0.0,
            end: 1.0,
        }
    }
}

impl CtrlnetStack {
    pub fn disable_all(&mut self) {
        self.switch_1 = "Off".into();
        self.controlnet_1 = "None".into();
        self.switch_2 = "Off".into();
        self.controlnet_2 = "None".into();
        self.switch_3 = "Off".into();
        self.controlnet_3 = "None".into();
    }

    pub fn enable(&mut self, idx: IdxControlNet, cfg: &CnCfg) {
        match idx {
            IdxControlNet::ControlNet1 => {
                self.switch_1 = "On".into();
                self.controlnet_1 = cfg.model.clone();
                self.controlnet_strength_1 = cfg.weight;
                self.start_percent_1 = cfg.start;
                self.end_percent_1 = cfg.end;
            }
            IdxControlNet::ControlNet2 => {
                self.switch_2 = "On".into();
                self.controlnet_2 = cfg.model.clone();
                self.controlnet_strength_2 = cfg.weight;
                self.start_percent_2 = cfg.start;
                self.end_percent_2 = cfg.end;
            }
            IdxControlNet::ControlNet3 => {
                self.switch_3 = "On".into();
                self.controlnet_3 = cfg.model.clone();
                self.controlnet_strength_3 = cfg.weight;
                self.start_percent_3 = cfg.start;
                self.end_percent_3 = cfg.end;
            }
        }
    }
}

impl Default for CtrlnetStack {
    fn default() -> Self {
        Self {
            switch_1: "Off".into(),
            controlnet_1: "None".into(),
            controlnet_strength_1: 1.0,
            start_percent_1: 0.0,
            end_percent_1: 1.0,
            switch_2: "Off".into(),
            controlnet_2: "None".into(),
            controlnet_strength_2: 1.0,
            start_percent_2: 0.0,
            end_percent_2: 1.0,
            switch_3: "Off".into(),
            controlnet_3: "None".into(),
            controlnet_strength_3: 1.0,
            start_percent_3: 0.0,
            end_percent_3: 1.0,
            image_1: None,
            image_2: None,
            image_3: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Ctrlnet {
    pub model: Vec<String>,
    pub preprocessor: Vec<String>,
}

#[cfg(test)]
mod comfy_tests {
    use std::collections::HashMap;

    use fixtures::control_nets;

    use super::*;

    #[test]
    fn cns_should_work() {
        let cns: HashMap<String, Ctrlnet> = serde_json::from_str(control_nets()).unwrap();
        assert!(cns.contains_key(CNT_CANNY));
        assert!(cns.contains_key(CNT_DEPTH));
        assert!(cns.contains_key(CNT_NORMAL_MAP));
        assert!(cns.contains_key(CNT_OPENPOSE));
        assert!(cns.contains_key(CNT_LINEART));
        assert!(cns.contains_key(CNT_ANIME_LINEART));
        assert!(cns.contains_key(CNT_SOFT_EDGE));
        assert!(cns.contains_key(CNT_SEGMENTATION));
        assert!(cns.contains_key(CNT_TILE));
    }
}
