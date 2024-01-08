mod error;
pub use error::*;

mod comfy;
pub use comfy::*;

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{info, trace};

    #[ctor::ctor]
    fn tests_setup() {
        tracing_subscriber::fmt::init();
        info!("tests_setup");
    }

    #[test]
    fn json_parsing_should_work() {
        let cn = ControlNetStack::default();
        let json = serde_json::to_string(&cn).unwrap();
        trace!("cn: {}", json);
        let cn2: ControlNetStack = serde_json::from_str(&json).unwrap();
        assert_eq!(cn.switch_1, cn2.switch_1);
    }

    #[test]
    fn workflow_parsing_should_work() {
        let wf = Workflow::from_file("test_workflow.json");
        trace!("wf: {:?}", wf);
        assert!(wf.is_ok());
        let wf = wf.unwrap();
        let node = wf.get_node("Efficient Loader").unwrap();
        trace!("node: {:?}", node);
    }
}
