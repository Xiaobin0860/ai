pub const NODE_CONTROL_NET_STACK_: &str = "CR Multi-ControlNet Stack";
pub const NODE_LORA_STACK: &str = "CR LoRA Stack";
pub const NODE_LOAD_IMAGE: &str = "LoadImage";
pub const NODE_SAVE_IMAGE: &str = "SaveImage";
pub const NODE_IMAGE_PREPROCESSOR: &str = "AIO_Preprocessor";
pub const NODE_EFFICIENT_LOADER: &str = "Efficient Loader";
pub const NODE_KSAMPLER: &str = "KSampler";

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

#[cfg(test)]
mod comfy_tests {
    use fixtures::test_workflow_api;
    use tracing::trace;

    use super::*;

    #[test]
    fn node_inputs_convert_should_work() {
        let cn = CtrlnetStack::default();
        let mut node = Node {
            inputs: Inputs::CtrlnetStack(cn.clone()),
            class_type: NODE_CONTROL_NET_STACK_.into(),
            meta: Meta {
                title: NODE_CONTROL_NET_STACK_.into(),
            },
        };
        assert_eq!(cn.switch_1, "Off");
        let cn2 = node.ctrlnet_stack();
        assert_eq!(cn.switch_1, cn2.switch_1);
        let mut cn3 = CtrlnetStack::from(&node);
        assert_eq!(cn.switch_1, cn3.switch_1);
        cn3.enable(IdxControlNet::ControlNet1, "cn1", 1.0, 0.0, 1.0);
        let cn4 = node.ctrlnet_stack_mut();
        cn4.enable(IdxControlNet::ControlNet1, "cn1", 1.0, 0.0, 1.0);
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
}
