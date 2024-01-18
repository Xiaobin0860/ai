pub const NODE_CTRLNET_STACK: &str = "CtrlnetStack";
pub const NODE_LORA_STACK: &str = "LoraStack";
pub const NODE_LOAD_IMAGE: &str = "LoadImage";
pub const NODE_SAVE_IMAGE: &str = "SaveImage";
pub const NODE_IMAGE_PREPROCESSOR: &str = "ImagePreprocessor";
pub const NODE_EFFICIENT_LOADER: &str = "EfficientLoader";
pub const NODE_KSAMPLER: &str = "KSampler";
pub const NODE_LINEART_PREPROCESSOR: &str = "LineArtPreprocessor";

mod api;
pub use api::*;

mod workflow;
pub use workflow::*;

mod node;
pub use node::*;

mod inputs;
pub use inputs::*;

mod lora;
pub use lora::*;

mod control_net;
pub use control_net::*;

mod image;
pub use image::*;

mod efficient;
pub use efficient::*;

mod sampler;
pub use sampler::*;

mod vae;
pub use vae::*;

mod preprocessor;
pub use preprocessor::*;

mod switch;
pub use switch::*;

mod latent;
pub use latent::*;

use std::collections::HashMap;
use std::sync::OnceLock;

pub fn class_map() -> &'static HashMap<&'static str, &'static str> {
    static HASHMAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    HASHMAP.get_or_init(|| serde_json::from_str(fixtures::class_names()).unwrap())
}

#[cfg(test)]
mod comfy_tests {
    use fixtures::{img2img, test_ai1, test_ai2, test_workflow_all, test_workflow_api, txt2img};
    use tracing::trace;

    use super::*;

    #[test]
    fn node_inputs_convert_should_work() {
        let cn = CtrlnetStack::default();
        let mut node = Node {
            inputs: Inputs::CtrlnetStack(cn.clone()),
            class_type: "c".into(),
            meta: Meta { title: "t".into() },
        };
        assert_eq!(cn.switch_1, "Off");
        let cn2 = node.ctrlnet_stack();
        assert_eq!(cn.switch_1, cn2.switch_1);
        let mut cn3 = CtrlnetStack::from(&node);
        assert_eq!(cn.switch_1, cn3.switch_1);
        let cfg = CnCfg::default();
        cn3.enable(IdxControlNet::ControlNet1, &cfg);
        let cn4 = node.ctrlnet_stack_mut();
        cn4.enable(IdxControlNet::ControlNet1, &cfg);
        assert_eq!(cn3.switch_1, "On");
        assert_eq!(cn4.switch_1, "On");
        cn3.disable_all();
        cn4.disable_all();
        assert_eq!(cn3.switch_1, "Off");
        assert_eq!(cn4.switch_1, "Off");
        assert_eq!(node.ctrlnet_stack().switch_1, "Off");
    }

    #[test]
    fn json_parsing_should_work() {
        let cn = CtrlnetStack::default();
        let json = serde_json::to_string(&cn).unwrap();
        trace!("cn: {}", json);
        let cn2: CtrlnetStack = serde_json::from_str(&json).unwrap();
        assert_eq!(cn.switch_1, cn2.switch_1);
    }

    #[test]
    fn workflow_parsing_should_work() {
        let wf = test_workflow_api();
        let wf = Workflow::from_json(wf);
        trace!("wf: {:?}", wf);
        assert!(wf.is_ok());
        let wf = wf.unwrap();
        let node = wf.get_node(NODE_EFFICIENT_LOADER).unwrap();
        trace!("node: {:?}", node);
    }

    #[test]
    fn all_workflow_parsing_should_work() {
        let wf = test_workflow_all();
        let wf = Workflow::from_json(wf);
        trace!("wf: {:?}", wf);
        assert!(wf.is_ok());
        let wf = wf.unwrap();
        let node = wf.get_node(NODE_EFFICIENT_LOADER).unwrap();
        trace!("node: {:?}", node);
    }

    #[test]
    fn ai_workflow_parsing_should_work() {
        let wf = test_ai1();
        let wf = Workflow::from_json(wf);
        println!("test_ai1: {:?}", wf);
        assert!(wf.is_ok());
        let wf = wf.unwrap();
        let node = wf.get_node(NODE_EFFICIENT_LOADER).unwrap();
        trace!("node: {:?}", node);

        let wf = test_ai2();
        let wf = Workflow::from_json(wf);
        println!("test_ai2: {:?}", wf);
    }

    #[test]
    fn api_parsing_should_work() {
        let wf = txt2img();
        let wf = Workflow::from_json(wf);
        println!("txt2img: {:?}", wf);
        assert!(wf.is_ok());

        let wf = img2img();
        let wf = Workflow::from_json(wf);
        println!("img2img: {:?}", wf);
        assert!(wf.is_ok());
    }
}
