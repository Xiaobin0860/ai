use std::fs;

use serde::Deserialize;

use crate::{AppResult, IdxControlNet};

#[derive(Debug, Deserialize)]
pub struct AutoCfg {
    pub ctrlnet_stack: Option<ACtrlnetStack>,

    pub efficient: Option<AEfficient>,

    pub load_image: Option<ALoadImage>,

    pub save_image: Option<ASaveImage>,

    pub lora_stack: Option<ALoraStack>,

    pub sampler: Option<ASampler>,
}

#[derive(Debug, Deserialize)]
pub struct ALoadImage {
    pub class_type: String,
    pub images: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ASaveImage {
    pub class_type: String,

    pub filename_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct ALoraStack {
    pub class_type: String,

    pub switch_1: bool,
    pub model_name_1: Vec<String>,
    pub strength_min_1: f32,
    pub strength_max_1: f32,

    pub switch_2: bool,
    pub model_name_2: Vec<String>,
    pub strength_min_2: f32,
    pub strength_max_2: f32,

    pub switch_3: bool,
    pub model_name_3: Vec<String>,
    pub strength_min_3: f32,
    pub strength_max_3: f32,
}

impl ALoraStack {
    pub fn switch(&self) -> bool {
        self.switch_1 || self.switch_2 || self.switch_3
    }
}

#[derive(Debug, Deserialize)]
pub struct ASampler {
    pub class_type: String,

    pub steps_min: u8,
    pub steps_max: u8,
    pub cfg_min: f32,
    pub cfg_max: f32,
    pub denoise_min: f32,
    pub denoise_max: f32,
    pub sampler_name: Vec<String>,
    pub scheduler: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ACtrlnetStack {
    pub class_type: String,

    pub switch_1: bool,
    pub ctrl_type_1: Vec<String>,
    pub start_min_1: f32,
    pub start_max_1: f32,
    pub end_min_1: f32,
    pub end_max_1: f32,
    pub strength_min_1: f32,
    pub strength_max_1: f32,

    //先只支持共用图片
    // pub images_1: Vec<String>,
    pub switch_2: bool,
    pub ctrl_type_2: Vec<String>,
    pub start_min_2: f32,
    pub start_max_2: f32,
    pub end_min_2: f32,
    pub end_max_2: f32,
    pub strength_min_2: f32,
    pub strength_max_2: f32,

    //先只支持共用图片
    // pub images_2: Vec<String>,
    pub switch_3: bool,
    pub ctrl_type_3: Vec<String>,
    pub start_min_3: f32,
    pub start_max_3: f32,
    pub end_min_3: f32,
    pub end_max_3: f32,
    pub strength_min_3: f32,
    pub strength_max_3: f32,
    // pub images_3: Vec<String>,
}

impl ACtrlnetStack {
    pub fn switch(&self) -> bool {
        self.switch_1 || self.switch_2 || self.switch_3
    }

    pub fn cfg(&self, idx: &IdxControlNet) -> Option<ACtrlnet> {
        match idx {
            IdxControlNet::ControlNet1 => {
                if self.switch_1 {
                    Some(ACtrlnet {
                        ctrl_type: self.ctrl_type_1.clone(),
                        start_min: self.start_min_1,
                        start_max: self.start_max_1,
                        end_min: self.end_min_1,
                        end_max: self.end_max_1,
                        strength_min: self.strength_min_1,
                        strength_max: self.strength_max_1,
                    })
                } else {
                    None
                }
            }
            IdxControlNet::ControlNet2 => {
                if self.switch_2 {
                    Some(ACtrlnet {
                        ctrl_type: self.ctrl_type_2.clone(),
                        start_min: self.start_min_2,
                        start_max: self.start_max_2,
                        end_min: self.end_min_2,
                        end_max: self.end_max_2,
                        strength_min: self.strength_min_2,
                        strength_max: self.strength_max_2,
                    })
                } else {
                    None
                }
            }
            IdxControlNet::ControlNet3 => {
                if self.switch_3 {
                    Some(ACtrlnet {
                        ctrl_type: self.ctrl_type_3.clone(),
                        start_min: self.start_min_3,
                        start_max: self.start_max_3,
                        end_min: self.end_min_3,
                        end_max: self.end_max_3,
                        strength_min: self.strength_min_3,
                        strength_max: self.strength_max_3,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ACtrlnet {
    pub ctrl_type: Vec<String>,
    pub start_min: f32,
    pub start_max: f32,
    pub end_min: f32,
    pub end_max: f32,
    pub strength_min: f32,
    pub strength_max: f32,
}

#[derive(Debug, Deserialize)]
pub struct AEfficient {
    pub class_type: String,
    pub positive: Vec<String>,
    pub negative: Vec<String>,
    pub batch_size: u16,
}

impl AutoCfg {
    pub fn from_toml(toml_str: &str) -> AppResult<Self> {
        Ok(toml::from_str(toml_str)?)
    }

    pub fn from_file(toml_file: &str) -> AppResult<Self> {
        Self::from_toml(fs::read_to_string(toml_file)?.as_str())
    }
}

#[cfg(test)]
mod ac_tests {
    use fixtures::test_auto_cfg;
    use tracing::trace;

    use crate::{
        NODE_CONTROL_NET_STACK_, NODE_EFFICIENT_LOADER, NODE_KSAMPLER, NODE_LOAD_IMAGE,
        NODE_LORA_STACK, NODE_SAVE_IMAGE,
    };

    use super::*;

    #[test]
    fn auto_cfg_should_work() {
        let cfg = test_auto_cfg();
        let cfg = AutoCfg::from_toml(cfg);
        trace!("{cfg:?}");
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        let ctrlnet_stack = &cfg.ctrlnet_stack.unwrap();
        assert!(ctrlnet_stack.switch_1);
        assert!(!ctrlnet_stack.ctrl_type_1.is_empty());
        assert_eq!(&ctrlnet_stack.class_type, NODE_CONTROL_NET_STACK_);
        assert_eq!(&cfg.efficient.unwrap().class_type, NODE_EFFICIENT_LOADER);
        assert_eq!(&cfg.save_image.unwrap().class_type, NODE_SAVE_IMAGE);
        assert_eq!(&cfg.sampler.unwrap().class_type, NODE_KSAMPLER);
        assert_eq!(cfg.load_image.unwrap().class_type, NODE_LOAD_IMAGE);
        let lora_stack = &cfg.lora_stack.unwrap();
        assert!(lora_stack.switch_1);
        assert!(!lora_stack.model_name_1.is_empty());
        assert_eq!(&lora_stack.class_type, NODE_LORA_STACK);
    }
}
