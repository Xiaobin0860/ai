mod error;
pub use error::*;

mod comfy;
pub use comfy::*;

#[cfg(test)]
mod tests {
    use tracing::info;

    #[ctor::ctor]
    fn tests_setup() {
        tracing_subscriber::fmt::init();
        info!("tests_setup");
    }
}
