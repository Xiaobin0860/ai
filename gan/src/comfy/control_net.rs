use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Inputs, Node};

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
    pub image_1: Vec<Value>,
    /// control net 2 image input
    pub image_2: Vec<Value>,
    /// control net 3 image input
    pub image_3: Vec<Value>,
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

    pub fn enable(&mut self, idx: IdxControlNet, name: &str, strength: f32, start: f32, end: f32) {
        match idx {
            IdxControlNet::ControlNet1 => {
                self.switch_1 = "On".into();
                self.controlnet_1 = name.into();
                self.controlnet_strength_1 = strength;
                self.start_percent_1 = start;
                self.end_percent_1 = end;
            }
            IdxControlNet::ControlNet2 => {
                self.switch_2 = "On".into();
                self.controlnet_2 = name.into();
                self.controlnet_strength_2 = strength;
                self.start_percent_2 = start;
                self.end_percent_2 = end;
            }
            IdxControlNet::ControlNet3 => {
                self.switch_3 = "On".into();
                self.controlnet_3 = name.into();
                self.controlnet_strength_3 = strength;
                self.start_percent_3 = start;
                self.end_percent_3 = end;
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
            image_1: vec![],
            image_2: vec![],
            image_3: vec![],
        }
    }
}

impl From<&Node> for CtrlnetStack {
    fn from(value: &Node) -> Self {
        match &value.inputs {
            Inputs::CtrlnetStack(v) => v.clone(),
            _ => panic!("CtrlnetStack"),
        }
    }
}
