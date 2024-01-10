mod error;
pub use error::*;

mod comfy;
pub use comfy::*;

mod app_args;
pub use app_args::*;

mod auto_cfg;
pub use auto_cfg::*;

#[cfg(test)]
mod tests {
    use tracing::info;

    #[ctor::ctor]
    fn tests_setup() {
        tracing_subscriber::fmt::init();
        info!("tests_setup");
    }
}
