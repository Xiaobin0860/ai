use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    /// comfy api url
    #[arg(long, default_value = "192.168.0.13:8188")]
    pub comfy_host: String,

    /// comfy ui workflow json file
    #[arg(long, default_value = "test_workflow.json")]
    pub workflow: String,

    /// auto generate config file
    #[arg(long, default_value = "auto_cfg.json")]
    pub auto_cfg: String,
}

impl Default for AppArgs {
    fn default() -> Self {
        Self::parse()
    }
}
