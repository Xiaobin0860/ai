use serde::{Deserialize, Serialize};

pub enum IdxLoRA {
    LoRA1,
    LoRA2,
    LoRA3,
}

/// LoRA stack
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoraStack {
    /// switch 1 On|Off
    pub switch_1: String,
    /// LoRA name 1
    pub lora_name_1: String,
    /// model weight 1
    pub model_weight_1: f32,
    /// clip weight 1
    pub clip_weight_1: f32,
    /// switch 2
    pub switch_2: String,
    /// LoRA name 2
    pub lora_name_2: String,
    /// model weight 2
    pub model_weight_2: f32,
    /// clip weight 2
    pub clip_weight_2: f32,
    /// switch 3
    pub switch_3: String,
    /// LoRA name 3
    pub lora_name_3: String,
    /// model weight 3
    pub model_weight_3: f32,
    /// clip weight 3
    pub clip_weight_3: f32,
}

impl LoraStack {
    pub fn disable_all(&mut self) {
        self.switch_1 = "Off".into();
        self.lora_name_1 = "None".into();
        self.switch_2 = "Off".into();
        self.lora_name_2 = "None".into();
        self.switch_3 = "Off".into();
        self.lora_name_3 = "None".into();
    }

    pub fn enable(&mut self, idx: IdxLoRA, name: &str, model_weight: f32, clip_weight: f32) {
        match idx {
            IdxLoRA::LoRA1 => {
                self.switch_1 = "On".into();
                self.lora_name_1 = name.into();
                self.model_weight_1 = model_weight;
                self.clip_weight_1 = clip_weight;
            }
            IdxLoRA::LoRA2 => {
                self.switch_2 = "On".into();
                self.lora_name_2 = name.into();
                self.model_weight_2 = model_weight;
                self.clip_weight_2 = clip_weight;
            }
            IdxLoRA::LoRA3 => {
                self.switch_3 = "On".into();
                self.lora_name_3 = name.into();
                self.model_weight_3 = model_weight;
                self.clip_weight_3 = clip_weight;
            }
        }
    }
}

impl Default for LoraStack {
    fn default() -> Self {
        Self {
            switch_1: "Off".into(),
            lora_name_1: "None".into(),
            model_weight_1: 1.0,
            clip_weight_1: 1.0,
            switch_2: "Off".into(),
            lora_name_2: "None".into(),
            model_weight_2: 1.0,
            clip_weight_2: 1.0,
            switch_3: "Off".into(),
            lora_name_3: "None".into(),
            model_weight_3: 1.0,
            clip_weight_3: 1.0,
        }
    }
}
