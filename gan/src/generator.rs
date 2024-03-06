use std::collections::HashMap;

use anyhow::{anyhow, Context};
use fixtures::control_nets;
use rand::{random, Rng};
use serde_json::Value;
use tracing::{debug, trace, warn};

use crate::{
    comfy_class_map, comfy_preprocessor, create_input_id, rand_element, ACtrlnet, ACtrlnetStack,
    AEmptyImage, AIPAdapter, AImageFilter, AImageRembg, ALoraStack, ALoraStacker, AppResult,
    AutoCfg, CnCfg, Ctrlnet, IdxControlNet, IdxLoRA, LoraCfg, LoraStack, LoraStacker, Workflow,
    NODE_CANNY_PREPROCESSOR, NODE_CROP_IMAGE, NODE_EMPTY_IMAGE, NODE_EMPTY_LATENT,
    NODE_IMAGE_FILTER, NODE_IMAGE_PREPROCESSOR, NODE_IMAGE_SCALESIDE, NODE_IMAGE_TAGGER,
    NODE_KSAMPLER, NODE_LINEARTSTANDARD_PREPROCESSOR, NODE_LINEART_PREPROCESSOR, NODE_LOAD_IMAGE,
    NODE_REPEAT_LATENT, NODE_TEXT_CONCAT, NODE_TEXT_STRING, NODE_TILE_PREPROCESSOR,
};

const STEP_F32: f32 = 0.05;

type TargetSize = (u16, u16);

pub struct Generator {
    cns: HashMap<String, Ctrlnet>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            cns: serde_json::from_str(control_nets()).unwrap(),
        }
    }

    pub fn rand(&self, wf: &mut Workflow, ac: &AutoCfg, idx: usize) -> AppResult<()> {
        self.rand_sampler(wf, ac)?;
        self.rand_lora(wf, ac)?;
        let land = self.rand_images(wf, ac, idx)?;
        let size = self.rand_efficient(wf, ac, land)?;
        self.rand_cn(wf, ac, size)?;
        if let Some(aif) = &ac.image_filter {
            self.apply_filter(wf, aif)?;
        }
        if let Some(arembg) = &ac.image_rembg {
            self.apply_rembg(wf, arembg)?;
        }
        if let Some(aipa) = &ac.ip_adapter {
            self.apply_ip_adapter(wf, aipa)?;
        }
        Ok(())
    }

    fn apply_ip_adapter(&self, wf: &mut Workflow, aipa: &AIPAdapter) -> AppResult<()> {
        if !aipa.switch {
            return Ok(());
        }
        let ipa_node = wf.get_node_mut(&aipa.title).context("ip_adapter")?;
        let ipa = ipa_node.i_p_adapter_apply_mut();
        let weight = rand_f32(aipa.weight_min, aipa.weight_max);
        let noise = rand_f32(aipa.noise_min, aipa.noise_max);
        let start = rand_f32(aipa.start_min, aipa.start_max);
        let end = rand_f32(aipa.end_min, aipa.end_max);
        let image = rand_element(&aipa.image);
        // TODO: 模型等其它参数基本不会变,先不实现了
        debug!("rand_ip_adapter: {weight}, {noise}, {start}, {end}, {image}");
        let img_id = ipa
            .image
            .first()
            .context("ip_adapter img")?
            .as_str()
            .context("ip_adapter img_id")?
            .to_owned();
        ipa.weight = weight;
        ipa.noise = noise;
        ipa.start_at = start;
        ipa.end_at = end;
        let ipa_id = ipa_node.id.clone();
        // KSampler.model = ipa
        wf.get_node_mut(NODE_KSAMPLER)
            .context("ip_adapter")?
            .k_sampler_mut()
            .model = Some(create_input_id(&ipa_id, 0));
        // ipa image
        wf.by_id_mut(&img_id)?.load_image_mut().image = image.clone();
        Ok(())
    }

    fn apply_rembg(&self, wf: &mut Workflow, arembg: &AImageRembg) -> AppResult<()> {
        if !arembg.switch {
            return Ok(());
        }
        let rembg_node = wf.get_node_mut(&arembg.title).context("rembg")?;
        let rembg = rembg_node.image_rembg_mut();
        rembg.model_name = arembg.model_name.clone();
        let rembg_id = rembg_node.id.clone();
        //新版本增加的`ImageRembg`结点,认为有一定有`CropImage`结点
        let crop = wf
            .get_node_mut(NODE_CROP_IMAGE)
            .context("rembg needs filter")?
            .image_filter_mut();
        crop.image = Some(create_input_id(&rembg_id, 0));
        Ok(())
    }

    fn apply_filter(&self, wf: &mut Workflow, aif: &AImageFilter) -> AppResult<()> {
        if !aif.switch {
            return Ok(());
        }
        let filter = wf.get_node_mut(&aif.title)?.image_filter_mut();
        if let Some(brightness) = aif.brightness {
            filter.brightness = brightness;
        }
        if let Some(contrast) = aif.contrast {
            filter.contrast = contrast;
        }
        if let Some(saturation) = aif.saturation {
            filter.saturation = saturation;
        }
        if let Some(sharpness) = aif.sharpness {
            filter.sharpness = sharpness;
        }
        if let Some(blur) = aif.blur {
            filter.blur = blur;
        }
        if let Some(gaussian_blur) = aif.gaussian_blur {
            filter.gaussian_blur = gaussian_blur;
        }
        if let Some(edge_enhance) = aif.edge_enhance {
            filter.edge_enhance = edge_enhance;
        }
        if let Some(detail_enhance) = &aif.detail_enhance {
            filter.detail_enhance = detail_enhance.clone();
        }
        Ok(())
    }

    fn rand_efficient(&self, wf: &mut Workflow, ac: &AutoCfg, land: bool) -> AppResult<TargetSize> {
        let ec = &ac.efficient;
        //land交换w,h
        let (w, h) = if land {
            (ec.height, ec.width)
        } else {
            (ec.width, ec.height)
        };
        //调整输入规格
        if let Ok(scale) = wf.get_node_mut(NODE_IMAGE_SCALESIDE) {
            let scale = scale.image_scale_side_mut();
            scale.side_length = if w > h { w } else { h };
        }
        let efficient = wf.get_node_mut(&ec.title)?.efficient_loader_mut();
        efficient.negative = rand_element(&ec.negative).clone();
        efficient.batch_size = ec.batch_size;
        efficient.empty_latent_width = w;
        efficient.empty_latent_height = h;
        efficient.vae_name = rand_element(&ec.vae_name).clone();
        efficient.ckpt_name = ec.ckpt_name.clone();
        efficient.clip_skip = *rand_element(&ec.clip_skip);
        efficient.weight_interpretation = ec.weight_interpretation.clone();
        let positive: &String = rand_element(&ec.positive);
        efficient.positive = Value::String(positive.into());
        //提示词及图标打标
        if let Ok(ts_node) = wf.get_node_mut(NODE_TEXT_STRING) {
            //支持Tagger的版本
            let ts = ts_node.text_string_mut();
            ts.text = positive.into();
            let mut ts_id = ts_node.id.clone();
            let atagger = ac.tagger.clone().unwrap_or_default();
            if atagger.switch {
                //有自动打标
                let if_id = wf.get_node_id(NODE_IMAGE_FILTER)?.clone();
                let tagger_node = wf.get_node_mut(&atagger.title)?;
                let tagger = tagger_node.tagger_mut();
                tagger.model = atagger.model.clone();
                //Tagger.image = ImageFilter
                tagger.image = Some(create_input_id(&if_id, 0));
                let tagger_id = tagger_node.id.clone();
                let concat_node = wf.get_node_mut(NODE_TEXT_CONCAT)?;
                let concat = concat_node.text_concat_mut();
                //TextConcat.text2 = Tagger
                concat.text2 = Some(create_input_id(&tagger_id, 0));
                ts_id = concat_node.id.clone();
                trace!("Tagger-{tagger_id}.image={if_id}, TextConcat{ts_id}.text2={tagger_id}");
            } else {
                //无自动打标, 移除Tagger结点
                wf.rem_node(NODE_IMAGE_TAGGER);
            }
            //EfficientLoader.positive = Text
            trace!("EfficientLoader.positive={ts_id}");
            wf.get_node_mut(&ec.title)?.efficient_loader_mut().positive =
                create_input_id(&ts_id, 0);
        }
        //图生图 用CropImage调整生图大小, 用RepeatLatent控制批次
        if let Ok(crop) = wf.get_node_mut(NODE_CROP_IMAGE) {
            let crop = crop.crop_image_mut();
            crop.target_w = w;
            crop.target_h = h;
            trace!("img2img: w={}, h={}", crop.target_w, crop.target_h);
        }
        if let Ok(repeat) = wf.get_node_mut(NODE_REPEAT_LATENT) {
            let repeat = repeat.repeat_latent_mut();
            repeat.amount = ec.batch_size;
            trace!("img2img: batch_size={}", repeat.amount);
        }
        //文生图 用EmptyLatent控制生图大小,批次
        if let Ok(latent) = wf.get_node_mut(NODE_EMPTY_LATENT) {
            let latent = latent.empty_latent_mut();
            latent.width = w;
            latent.height = h;
            latent.batch_size = ec.batch_size;
            trace!("txt2img: {latent:?}");
        }
        //图生图纯色图输入
        if let Some(aemc) = &ac.empty_image {
            self.apply_empty(aemc, (w, h), ec.batch_size, wf);
        }

        Ok((w, h))
    }

    fn apply_empty(&self, aemc: &AEmptyImage, (w, h): TargetSize, bs: u8, wf: &mut Workflow) {
        if !aemc.switch {
            return;
        }
        if let Ok(empty) = wf.get_node_mut(NODE_EMPTY_IMAGE) {
            let empty = empty.empty_image_mut();
            empty.width = w;
            empty.height = h;
            empty.batch_size = bs;
            empty.color = aemc.color;
            trace!("img2img: {empty:?}");
        } else {
            warn!("{NODE_EMPTY_IMAGE} not found");
        }
    }

    // 图片名以`land_`开头, 返回`true`
    fn rand_images(&self, wf: &mut Workflow, ac: &AutoCfg, idx: usize) -> AppResult<bool> {
        let mut land = false;
        let img_name = if let Some(cfg) = &ac.load_image {
            let imgs = &cfg.images;
            let img_name = imgs.get(idx % imgs.len()).context("get img")?;
            if img_name.starts_with("land_") {
                land = true;
            }
            wf.get_node_mut(NODE_LOAD_IMAGE)?.load_image_mut().image = img_name.clone();
            img_name
        } else {
            &ac.save_image.filename_prefix
        };
        let save = &ac.save_image;
        let saver = wf.get_node_mut(&save.title)?.image_save_mut();
        saver.filename_prefix = img_name
            .split('.')
            .next()
            .context("get img_name")?
            .to_owned();
        saver.output_path = save.output_path.clone();
        Ok(land)
    }

    fn rand_cn(&self, wf: &mut Workflow, ac: &AutoCfg, size: TargetSize) -> AppResult<()> {
        if let Some(acn) = &ac.ctrlnet_stack {
            if acn.switch() {
                wf.get_node_mut(acn.title.as_str())?
                    .ctrlnet_stack_mut()
                    .disable_all();

                self.rand_cn1(wf, acn, size)?;
                self.rand_cn2(wf, acn, size)?;
                self.rand_cn3(wf, acn, size)?;
            }
        }
        Ok(())
    }

    fn rand_preprocessor(
        &self,
        wf: &mut Workflow,
        cfg: &CnCfg,
        acn_stack: &ACtrlnetStack,
    ) -> AppResult<String> {
        //preprocessor特殊处理
        let my_processor_name = *comfy_class_map()
            .get(cfg.preprocessor.as_str())
            .unwrap_or(&NODE_IMAGE_PREPROCESSOR);
        let processor_node = wf.get_node_mut(my_processor_name)?;
        match my_processor_name {
            NODE_LINEART_PREPROCESSOR => {
                // realistic|coarse
                let processor = processor_node.line_art_preprocessor_mut();
                match cfg.my_name.as_str() {
                    "realistic" => {
                        processor.coarse = "disable".into();
                    }
                    "coarse" => {
                        processor.coarse = "enable".into();
                    }
                    _ => {
                        processor.coarse = if random::<bool>() {
                            "enable".into()
                        } else {
                            "disable".into()
                        };
                    }
                }
                processor.coarse = if random::<bool>() {
                    "enable".into()
                } else {
                    "disable".into()
                };
                processor.resolution = cfg.resolution;
            }
            NODE_LINEARTSTANDARD_PREPROCESSOR => {
                // resolution
                let processor = processor_node.lineart_standard_preprocessor_mut();
                processor.resolution = cfg.resolution;
            }
            NODE_TILE_PREPROCESSOR => {
                // resolution
                let processor = processor_node.tile_preprocessor_mut();
                processor.resolution = cfg.resolution;
                processor.pyrup_iters = if acn_stack.tile_pyrup_iters > 0 {
                    acn_stack.tile_pyrup_iters
                } else {
                    random::<u8>() % 3 + 1
                };
            }
            NODE_IMAGE_PREPROCESSOR => {
                // resolution
                let processor = processor_node.image_preprocessor_mut();
                processor.resolution = cfg.resolution;
                processor.preprocessor = cfg.preprocessor.clone();
            }
            NODE_CANNY_PREPROCESSOR => {
                let processor = processor_node.canny_edge_preprocessor_mut();
                processor.resolution = cfg.resolution;
                if acn_stack.canny_low_threshold > 0 {
                    processor.low_threshold = acn_stack.canny_low_threshold;
                    processor.high_threshold = acn_stack.canny_high_threshold;
                } else {
                    processor.low_threshold = *rand_element(&[40, 60, 80, 100]);
                    processor.high_threshold = processor.low_threshold * 2;
                }
            }
            _ => {
                warn!("unhandled preprocessor {my_processor_name} {cfg:?}");
            }
        }
        Ok(processor_node.id.clone())
    }

    fn rand_cn1(&self, wf: &mut Workflow, acn: &ACtrlnetStack, size: TargetSize) -> AppResult<()> {
        let idx = IdxControlNet::ControlNet1;
        if let Some(acfg) = acn.cfg(&idx) {
            let cfg = self.rand_cn_cfg(&acfg, size)?;
            debug!("rand_cn1 acfg={acfg:?}, cn_cfg={cfg:?}");
            wf.get_node_mut(acn.title.as_str())?
                .ctrlnet_stack_mut()
                .enable(idx, &cfg);
            let id = self.rand_preprocessor(wf, &cfg, acn)?;
            wf.get_node_mut(acn.title.as_str())?
                .ctrlnet_stack_mut()
                .image_1 = Some(create_input_id(&id, 0));
        }
        Ok(())
    }

    fn rand_cn2(&self, wf: &mut Workflow, acn: &ACtrlnetStack, size: TargetSize) -> AppResult<()> {
        let idx = IdxControlNet::ControlNet2;
        if let Some(acfg) = acn.cfg(&idx) {
            let cfg = self.rand_cn_cfg(&acfg, size)?;
            debug!("rand_cn2 acfg={acfg:?}, cn_cfg={cfg:?}");
            wf.get_node_mut(acn.title.as_str())?
                .ctrlnet_stack_mut()
                .enable(idx, &cfg);
            let id = self.rand_preprocessor(wf, &cfg, acn)?;
            wf.get_node_mut(acn.title.as_str())?
                .ctrlnet_stack_mut()
                .image_2 = Some(create_input_id(&id, 0));
        }
        Ok(())
    }

    fn rand_cn3(&self, wf: &mut Workflow, acn: &ACtrlnetStack, size: TargetSize) -> AppResult<()> {
        let idx = IdxControlNet::ControlNet3;
        if let Some(acfg) = acn.cfg(&idx) {
            let cfg = self.rand_cn_cfg(&acfg, size)?;
            debug!("rand_cn3 acfg={acfg:?}, cn_cfg={cfg:?}");
            wf.get_node_mut(acn.title.as_str())?
                .ctrlnet_stack_mut()
                .enable(idx, &cfg);
            let id = self.rand_preprocessor(wf, &cfg, acn)?;
            wf.get_node_mut(acn.title.as_str())?
                .ctrlnet_stack_mut()
                .image_3 = Some(create_input_id(&id, 0));
        }
        Ok(())
    }

    fn rand_cn_cfg(&self, acfg: &ACtrlnet, size: TargetSize) -> AppResult<CnCfg> {
        let ctrl_type = rand_element(&acfg.ctrl_type);
        let cn = self
            .cns
            .get(ctrl_type)
            .ok_or(anyhow!("no cn type {ctrl_type}"))?;
        let weight = rand_f32(acfg.strength_min, acfg.strength_max);
        let start = rand_f32(acfg.start_min, acfg.start_max);
        let end = rand_f32(acfg.end_min, acfg.end_max);
        let preprocessors = if acfg.preprocessor.is_empty() {
            //所有可用的preprocessor随机, 注意这是comfy的preprocessor名
            &cn.preprocessor
        } else {
            //指定的preprocessor随机, 注意这是显示可读的preprocessor名
            &acfg.preprocessor
        };
        trace!("preprocessor: {preprocessors:?}");
        let my_name = rand_element(preprocessors).to_owned();
        let preprocessor = comfy_preprocessor(&my_name).to_owned();
        let resolution = *rand_element(&acfg.resolution);
        let resolution = if resolution > 0 { resolution } else { size.0 };
        debug!("rand_cn_cfg: {ctrl_type}, {weight}, {start}, {end}, {resolution}, {my_name}-{preprocessor}");
        Ok(CnCfg {
            model: rand_element(&cn.model).clone(),
            preprocessor,
            weight,
            start,
            end,
            resolution,
            my_name,
        })
    }

    fn rand_sampler(&self, wf: &mut Workflow, ac: &AutoCfg) -> AppResult<()> {
        let sampler = wf.get_node_mut(NODE_KSAMPLER)?.k_sampler_mut();
        sampler.seed = random::<u32>() as i64;
        let asampler = &ac.sampler;
        let steps = rand_num(asampler.steps_min, asampler.steps_max);
        let cfg = rand_f32(asampler.cfg_min, asampler.cfg_max);
        let denoise = rand_f32(asampler.denoise_min, asampler.denoise_max);
        let sampler_name = rand_element(&asampler.sampler_name);
        let scheduler = rand_element(&asampler.scheduler);
        debug!("rand_sampler: {steps}, {cfg}, {denoise}, {sampler_name}-{scheduler}");
        sampler.steps = steps;
        sampler.cfg = cfg;
        sampler.denoise = denoise;
        sampler.sampler_name = sampler_name.clone();
        sampler.scheduler = scheduler.clone();
        Ok(())
    }

    fn rand_lora(&self, wf: &mut Workflow, ac: &AutoCfg) -> AppResult<()> {
        if let Some(alora) = &ac.lora_stack {
            let lora_stack = wf.get_node_mut(alora.title.as_str())?.lora_stack_mut();
            lora_stack.disable_all();
            if alora.switch() {
                self.rand_lora1(lora_stack, alora);
                self.rand_lora2(lora_stack, alora);
                self.rand_lora3(lora_stack, alora);
            }
        }
        if let Some(astacker) = &ac.lora_stacker {
            let stacker = wf.get_node_mut(astacker.title.as_str())?.lora_stacker_mut();
            stacker.disable_all();
            stacker.lora_count = astacker.lora_count;
            for i in 1..=astacker.lora_count {
                self.rand_loran(i, astacker, stacker);
            }
        }
        Ok(())
    }

    fn rand_loran(&self, i: u8, alora: &ALoraStacker, stacker: &mut LoraStacker) {
        match i {
            1 => {
                let cfg = rand_lora_cfg(
                    &alora.model_name_1,
                    alora.strength_min_1,
                    alora.strength_max_1,
                );
                stacker.enable(IdxLoRA::LoRA1, &cfg);
            }
            2 => {
                let cfg = rand_lora_cfg(
                    &alora.model_name_2,
                    alora.strength_min_2,
                    alora.strength_max_2,
                );
                stacker.enable(IdxLoRA::LoRA2, &cfg);
            }
            3 => {
                let cfg = rand_lora_cfg(
                    &alora.model_name_3,
                    alora.strength_min_3,
                    alora.strength_max_3,
                );
                stacker.enable(IdxLoRA::LoRA3, &cfg);
            }
            4 => {
                let cfg = rand_lora_cfg(
                    &alora.model_name_4,
                    alora.strength_min_4,
                    alora.strength_max_4,
                );
                stacker.enable(IdxLoRA::LoRA4, &cfg);
            }
            _ => {
                warn!("unhandled lora {i}");
            }
        }
    }

    fn rand_lora1(&self, lora_stack: &mut LoraStack, alora: &ALoraStack) {
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

    fn rand_lora2(&self, lora_stack: &mut LoraStack, alora: &ALoraStack) {
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

    fn rand_lora3(&self, lora_stack: &mut LoraStack, alora: &ALoraStack) {
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
    let model_weight = rand_f32(wmodel_min, wmodel_max);
    LoraCfg {
        lora_name: rand_element(names).clone(),
        model_weight,
        clip_weight: model_weight,
    }
}

fn rand_f32(min: f32, max: f32) -> f32 {
    if max - min > STEP_F32 {
        let mut vs = Vec::new();
        let mut max = max;
        vs.push(max);
        while max > min {
            max -= STEP_F32;
            if max < min {
                vs.push(min);
                break;
            }
            vs.push(max);
        }
        trace!("rand_f32: {vs:?}");
        *rand_element(&vs)
    } else {
        max
    }
}

fn rand_num<T>(min: T, max: T) -> T
where
    T: rand::distributions::uniform::SampleUniform + PartialOrd,
{
    let mut rng = rand::thread_rng();
    if max > min {
        rng.gen_range(min..=max)
    } else {
        max
    }
}
