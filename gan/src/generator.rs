use std::collections::HashMap;

use anyhow::anyhow;
use fixtures::control_nets;
use rand::random;
use serde_json::Value;
use tracing::debug;

use crate::{
    rand_element, ACtrlnet, ACtrlnetStack, ALoraStack, AppResult, AutoCfg, CnCfg, Ctrlnet,
    CtrlnetStack, IdxControlNet, IdxLoRA, LoraCfg, LoraStack, Workflow, NODE_KSAMPLER,
    NODE_LINEARTPREPROCESSOR, NODE_LOAD_IMAGE,
};

pub struct Generator {
    pub seed: i64,
    cns: HashMap<String, Ctrlnet>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            seed: -1,
            cns: serde_json::from_str(control_nets()).unwrap(),
        }
    }

    pub fn rand(&mut self, wf: &mut Workflow, ac: &AutoCfg) -> AppResult<Value> {
        self.rand_sampler(wf, ac)?;
        self.rand_lora(wf, ac)?;
        self.rand_cn(wf, ac)?;
        self.rand_images(wf, ac)?;
        wf.to_json()
    }

    fn rand_images(&mut self, wf: &mut Workflow, ac: &AutoCfg) -> AppResult<()> {
        if let Some(cfg) = &ac.load_image {
            let imgs = &cfg.images;
            let img_name = rand_element(imgs);
            wf.get_node_mut(NODE_LOAD_IMAGE)?.load_image_mut().image = img_name.clone();
            if let Some(save) = &ac.save_image {
                wf.get_node_mut(&save.class_type)?
                    .save_image_mut()
                    .filename_prefix = img_name.split('.').next().unwrap().to_owned();
            }
        }
        Ok(())
    }

    fn rand_cn(&self, wf: &mut Workflow, ac: &AutoCfg) -> AppResult<()> {
        if let Some(acn) = &ac.ctrlnet_stack {
            if acn.switch() {
                let cn_stack = wf
                    .get_node_mut(acn.class_type.as_str())?
                    .ctrlnet_stack_mut();
                cn_stack.disable_all();
                if let Some(cfg) = self.rand_cn1(cn_stack, acn)? {
                    //preprocessor
                    //TODO: rand preprocessor
                    if cfg.preprocessor == NODE_LINEARTPREPROCESSOR {
                        wf.get_node_mut(&cfg.preprocessor)?
                            .line_art_preprocessor_mut()
                            .coarse = if random::<bool>() {
                            "enable".into()
                        } else {
                            "disable".into()
                        };
                    }
                }
                // self.rand_cn2(cn_stack, acn);
                // self.rand_cn3(cn_stack, acn);
            }
        }
        Ok(())
    }

    fn rand_cn1(
        &self,
        cn_stack: &mut CtrlnetStack,
        acn: &ACtrlnetStack,
    ) -> AppResult<Option<CnCfg>> {
        let idx = IdxControlNet::ControlNet1;
        if let Some(acfg) = acn.cfg(&idx) {
            let cfg = self.rand_cn_cfg(&acfg)?;
            debug!("rand_cn1 acfg={acfg:?}, cn_cfg={cfg:?}");
            cn_stack.enable(idx, &cfg);
            Ok(Some(cfg))
        } else {
            Ok(None)
        }
    }

    fn rand_cn_cfg(&self, acfg: &ACtrlnet) -> AppResult<CnCfg> {
        let ctrl_type = rand_element(&acfg.ctrl_type);
        let cn = self
            .cns
            .get(ctrl_type)
            .ok_or(anyhow!("no cn type {ctrl_type}"))?;
        Ok(CnCfg {
            model: rand_element(&cn.model).clone(),
            preprocessor: rand_element(&cn.preprocessor).clone(),
            weight: random::<f32>() * (acfg.strength_max - acfg.strength_min) + acfg.strength_min,
            start: random::<f32>() * (acfg.start_max - acfg.start_min) + acfg.start_min,
            end: random::<f32>() * (acfg.end_max - acfg.end_min) + acfg.end_min,
            ..Default::default()
        })
    }

    fn rand_sampler(&mut self, wf: &mut Workflow, ac: &AutoCfg) -> AppResult<()> {
        if -1 == self.seed {
            self.seed = random::<u32>() as i64;
        }
        let sampler = wf.get_node_mut(NODE_KSAMPLER)?.k_sampler_mut();
        sampler.seed = self.seed;
        if let Some(asampler) = &ac.sampler {
            //rand [steps_min, steps_max]
            sampler.steps =
                random::<u8>() % (asampler.steps_max - asampler.steps_min + 1) + asampler.steps_min;
            //rand [cfg_min, cfg_max]
            sampler.cfg =
                random::<f32>() * (asampler.cfg_max - asampler.cfg_min) + asampler.cfg_min;
            //rand [denoise_min, denoise_max]
            sampler.denoise = random::<f32>() * (asampler.denoise_max - asampler.denoise_min)
                + asampler.denoise_min;
        }
        Ok(())
    }

    fn rand_lora(&mut self, wf: &mut Workflow, ac: &AutoCfg) -> AppResult<()> {
        if let Some(alora) = &ac.lora_stack {
            if alora.switch() {
                let lora_stack = wf.get_node_mut(alora.class_type.as_str())?.lora_stack_mut();
                lora_stack.disable_all();
                self.rand_lora1(lora_stack, alora);
                self.rand_lora2(lora_stack, alora);
                self.rand_lora3(lora_stack, alora);
            }
        }
        Ok(())
    }

    fn rand_lora1(&mut self, lora_stack: &mut LoraStack, alora: &ALoraStack) {
        if !alora.switch_1 {
            return;
        }
        let cfg = rand_lora_cfg(
            &alora.model_name_1,
            alora.strength_min_1,
            alora.strength_max_1,
        );
        lora_stack.enable(IdxLoRA::LoRA1, &cfg);
    }

    fn rand_lora2(&mut self, lora_stack: &mut LoraStack, alora: &ALoraStack) {
        if !alora.switch_2 {
            return;
        }
        let cfg = rand_lora_cfg(
            &alora.model_name_2,
            alora.strength_min_2,
            alora.strength_max_2,
        );
        lora_stack.enable(IdxLoRA::LoRA2, &cfg);
    }

    fn rand_lora3(&mut self, lora_stack: &mut LoraStack, alora: &ALoraStack) {
        if !alora.switch_3 {
            return;
        }
        let cfg = rand_lora_cfg(
            &alora.model_name_3,
            alora.strength_min_3,
            alora.strength_max_3,
        );
        lora_stack.enable(IdxLoRA::LoRA3, &cfg);
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

fn rand_lora_cfg(names: &[String], wmodel_min: f32, wmodel_max: f32) -> LoraCfg {
    LoraCfg {
        lora_name: rand_element(names).clone(),
        model_weight: random::<f32>() * (wmodel_max - wmodel_min) + wmodel_min,
        ..Default::default()
    }
}