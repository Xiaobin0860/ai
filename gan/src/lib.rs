mod error;
pub use error::*;

mod comfy;
pub use comfy::*;

mod app_args;
pub use app_args::*;

mod auto_cfg;
pub use auto_cfg::*;

mod generator;
pub use generator::*;

pub fn rand_element<T>(v: &[T]) -> &T {
    debug_assert!(!v.is_empty());
    if v.len() == 1 {
        v.first().unwrap()
    } else {
        let idx = rand::random::<usize>() % v.len();
        v.get(idx).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;

    #[ctor::ctor]
    fn tests_setup() {
        tracing_subscriber::fmt::init();
        info!("tests_setup");
    }
}
