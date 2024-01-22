use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    #[arg(long, default_value = "app_cfg.toml")]
    pub cfg: String,

    #[arg(long)]
    pub clean: bool,
}

impl Default for AppArgs {
    fn default() -> Self {
        Self::parse()
    }
}
