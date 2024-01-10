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

include_str_as_fn!(control_nets, "../data/control_nets.json");
