use std::fs;

use serde::Deserialize;

use crate::AppResult;

#[derive(Debug, Deserialize)]
pub struct AutoCfg {
    pub ctrlnet_stack: ACtrlnetStack,

    pub efficient: AEfficient,

    pub load_image: ALoadImage,

    pub save_image: ASaveImage,

    pub sampler: ASampler,
}

#[derive(Debug, Deserialize)]
pub struct ALoadImage {
    pub class_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ASaveImage {
    pub class_type: String,

    pub filename_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct ASampler {
    pub class_type: String,

    pub steps_min: u16,
    pub steps_max: u16,
    pub cfg_min: f32,
    pub cfg_max: f32,
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
    pub images_1: Vec<String>,

    pub switch_2: bool,
    pub ctrl_type_2: Vec<String>,
    pub start_min_2: f32,
    pub start_max_2: f32,
    pub end_min_2: f32,
    pub end_max_2: f32,
    pub strength_min_2: f32,
    pub strength_max_2: f32,
    pub images_2: Vec<String>,

    pub switch_3: bool,
    pub ctrl_type_3: Vec<String>,
    pub start_min_3: f32,
    pub start_max_3: f32,
    pub end_min_3: f32,
    pub end_max_3: f32,
    pub strength_min_3: f32,
    pub strength_max_3: f32,
    pub images_3: Vec<String>,
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

    use crate::{NODE_CONTROL_NET_STACK_, NODE_EFFICIENT_LOADER, NODE_KSAMPLER, NODE_SAVE_IMAGE};

    use super::*;

    #[test]
    fn auto_cfg_should_work() {
        let cfg = test_auto_cfg();
        let cfg = AutoCfg::from_toml(cfg);
        trace!("{cfg:?}");
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert!(cfg.ctrlnet_stack.switch_1);
        assert!(!cfg.ctrlnet_stack.ctrl_type_1.is_empty());
        assert_eq!(&cfg.ctrlnet_stack.class_type, NODE_CONTROL_NET_STACK_);
        assert_eq!(&cfg.efficient.class_type, NODE_EFFICIENT_LOADER);
        assert_eq!(&cfg.save_image.class_type, NODE_SAVE_IMAGE);
        assert_eq!(&cfg.sampler.class_type, NODE_KSAMPLER);
    }
}
