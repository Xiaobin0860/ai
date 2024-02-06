use std::fs;

use serde::{Deserialize, Serialize};

use crate::{AppResult, IdxControlNet};

/// 应用状态
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    #[serde(skip)]
    db_file: String,

    /// 当前跑到第几个配置 [0, auto_cfgs.len)
    #[serde(default)]
    pub cfg_idx: usize,

    /// 当前配置跑到第几个循环 [0, total)
    #[serde(default)]
    pub total_idx: usize,
}
impl AppState {
    fn from_json(toml_str: &str) -> AppResult<Self> {
        Ok(serde_json::from_str(toml_str)?)
    }

    pub fn from_file(db_file: &str) -> AppResult<Self> {
        let content = fs::read_to_string(db_file).unwrap_or("{}".to_owned());
        let mut s = Self::from_json(content.as_str())?;
        s.db_file = db_file.to_owned();
        Ok(s)
    }

    pub fn clean(&mut self) {
        self.cfg_idx = 0;
        self.total_idx = 0;
    }

    pub fn save(&self) -> AppResult<()> {
        let content = serde_json::to_string(self)?;
        fs::write(self.db_file.as_str(), content)?;
        Ok(())
    }

    pub fn update(&mut self, cfg_idx: usize, total_idx: usize) -> AppResult<()> {
        self.cfg_idx = cfg_idx;
        self.total_idx = total_idx;
        self.save()
    }
}

#[derive(Debug, Deserialize)]
pub struct AppCfg {
    pub comfy_api: String,
    pub auto_cfgs: Vec<String>,
}

impl AppCfg {
    pub fn from_toml(toml_str: &str) -> AppResult<Self> {
        Ok(toml::from_str(toml_str)?)
    }

    pub fn from_file(toml_file: &str) -> AppResult<Self> {
        Self::from_toml(fs::read_to_string(toml_file)?.as_str())
    }
}

/// 自动出图配置
#[derive(Debug, Deserialize)]
pub struct AutoCfg {
    /// 工作流
    pub workflows: Vec<String>,

    /// 总出图数
    pub total: usize,

    /// 每套参数出图数(只变化种子)
    pub ct_per_params: usize,

    /// efficient配置 chkpt_name, vae_name, clip_skip, positive, negative, w,h
    pub efficient: AEfficient,

    /// 采样器配置 seed, steps, cfg, denoise, sampler_name, scheduler
    pub sampler: ASampler,

    /// 图片保存
    pub save_image: ASaveImage,

    /// 图片加载
    pub load_image: Option<ALoadImage>,

    /// 图片处理
    pub image_filter: Option<AImageFilter>,

    /// CN配置
    pub ctrlnet_stack: Option<ACtrlnetStack>,

    /// lora配置
    pub lora_stack: Option<ALoraStack>,
    pub lora_stacker: Option<ALoraStacker>,

    /// 图生图输入纯色图
    pub empty_image: Option<AEmptyImage>,

    /// 图片去背景
    pub image_rembg: Option<AImageRembg>,
}

#[derive(Debug, Deserialize)]
pub struct AEmptyImage {
    pub title: String,
    pub switch: bool,
    pub color: u32,
}

#[derive(Debug, Deserialize)]
pub struct AImageRembg {
    pub title: String,
    pub switch: bool,
    pub model_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ALoadImage {
    pub title: String,
    pub images: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AImageFilter {
    pub title: String,
    pub brightness: Option<f32>,
    pub contrast: Option<f32>,
    pub saturation: Option<f32>,
    pub sharpness: Option<f32>,
    /// [0, 16]
    pub blur: Option<u8>,
    pub gaussian_blur: Option<f32>,
    pub edge_enhance: Option<f32>,
    pub detail_enhance: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ASaveImage {
    pub title: String,
    pub output_path: String,
    pub filename_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct ALoraStack {
    pub title: String,

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
pub struct ALoraStacker {
    pub title: String,
    pub lora_count: u8,

    pub model_name_1: Vec<String>,
    pub strength_min_1: f32,
    pub strength_max_1: f32,

    pub model_name_2: Vec<String>,
    pub strength_min_2: f32,
    pub strength_max_2: f32,

    pub model_name_3: Vec<String>,
    pub strength_min_3: f32,
    pub strength_max_3: f32,

    pub model_name_4: Vec<String>,
    pub strength_min_4: f32,
    pub strength_max_4: f32,
}

#[derive(Debug, Deserialize)]
pub struct ASampler {
    pub title: String,

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
    pub title: String,

    pub canny_low_threshold: u8,
    pub canny_high_threshold: u8,
    pub tile_pyrup_iters: u8,

    pub switch_1: bool,
    pub ctrl_type_1: Vec<String>,
    pub processor_1: Vec<String>,
    //0.05
    pub start_min_1: f32,
    pub start_max_1: f32,
    pub end_min_1: f32,
    pub end_max_1: f32,
    //0.5
    pub strength_min_1: f32,
    pub strength_max_1: f32,
    pub resolution_1: Vec<u16>,

    //先只支持共用图片
    // pub images_1: Vec<String>,
    pub switch_2: bool,
    pub ctrl_type_2: Vec<String>,
    pub processor_2: Vec<String>,
    pub start_min_2: f32,
    pub start_max_2: f32,
    pub end_min_2: f32,
    pub end_max_2: f32,
    pub strength_min_2: f32,
    pub strength_max_2: f32,
    pub resolution_2: Vec<u16>,

    pub switch_3: bool,
    pub ctrl_type_3: Vec<String>,
    pub processor_3: Vec<String>,
    pub start_min_3: f32,
    pub start_max_3: f32,
    pub end_min_3: f32,
    pub end_max_3: f32,
    pub strength_min_3: f32,
    pub strength_max_3: f32,
    pub resolution_3: Vec<u16>,
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
                        preprocessor: self.processor_1.clone(),
                        start_min: self.start_min_1,
                        start_max: self.start_max_1,
                        end_min: self.end_min_1,
                        end_max: self.end_max_1,
                        strength_min: self.strength_min_1,
                        strength_max: self.strength_max_1,
                        resolution: self.resolution_1.clone(),
                    })
                } else {
                    None
                }
            }
            IdxControlNet::ControlNet2 => {
                if self.switch_2 {
                    Some(ACtrlnet {
                        ctrl_type: self.ctrl_type_2.clone(),
                        preprocessor: self.processor_2.clone(),
                        start_min: self.start_min_2,
                        start_max: self.start_max_2,
                        end_min: self.end_min_2,
                        end_max: self.end_max_2,
                        strength_min: self.strength_min_2,
                        strength_max: self.strength_max_2,
                        resolution: self.resolution_2.clone(),
                    })
                } else {
                    None
                }
            }
            IdxControlNet::ControlNet3 => {
                if self.switch_3 {
                    Some(ACtrlnet {
                        ctrl_type: self.ctrl_type_3.clone(),
                        preprocessor: self.processor_3.clone(),
                        start_min: self.start_min_3,
                        start_max: self.start_max_3,
                        end_min: self.end_min_3,
                        end_max: self.end_max_3,
                        strength_min: self.strength_min_3,
                        strength_max: self.strength_max_3,
                        resolution: self.resolution_3.clone(),
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
    pub preprocessor: Vec<String>,
    pub start_min: f32,
    pub start_max: f32,
    pub end_min: f32,
    pub end_max: f32,
    pub strength_min: f32,
    pub strength_max: f32,
    pub resolution: Vec<u16>,
}

#[derive(Debug, Deserialize)]
pub struct AEfficient {
    pub title: String,
    pub positive: Vec<String>,
    pub negative: Vec<String>,
    pub width: u16,
    pub height: u16,
    pub batch_size: u8,
    pub vae_name: Vec<String>,
    pub clip_skip: Vec<i8>,
    pub ckpt_name: String,
    pub weight_interpretation: String,
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
        NODE_CTRLNET_STACK, NODE_EFFICIENT_LOADER, NODE_KSAMPLER, NODE_LOAD_IMAGE, NODE_SAVE_IMAGE,
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
        assert_eq!(&ctrlnet_stack.title, NODE_CTRLNET_STACK);
        assert_eq!(&cfg.efficient.title, NODE_EFFICIENT_LOADER);
        assert_eq!(&cfg.save_image.title, NODE_SAVE_IMAGE);
        assert_eq!(&cfg.sampler.title, NODE_KSAMPLER);
        assert_eq!(cfg.load_image.unwrap().title, NODE_LOAD_IMAGE);
    }

    #[test]
    fn state_should_work() {
        let state = AppState::from_file("nonononononono").unwrap();
        assert_eq!(0, state.cfg_idx);
        assert_eq!(0, state.total_idx);
    }
}
