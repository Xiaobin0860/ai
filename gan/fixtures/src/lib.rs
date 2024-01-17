#[macro_export]
macro_rules! include_str_as_fn {
    ($name:ident, $file:expr) => {
        pub fn $name() -> &'static str {
            include_str!($file)
        }
    };
}

include_str_as_fn!(test_auto_cfg, "../data/test_auto_cfg.toml");
include_str_as_fn!(test_workflow_api, "../data/test_workflow_api.json");
include_str_as_fn!(test_workflow_all, "../data/test_workflow_all.json");
include_str_as_fn!(test_ai1, "../data/ai改画-素描-石膏像api.json");
include_str_as_fn!(test_ai2, "../data/ai改画-素描-几何体.json");

include_str_as_fn!(control_nets, "../data/control_nets.json");
include_str_as_fn!(meikao, "../data/美考api.json");
