use tracing::trace;

fn main() {
    tracing_subscriber::fmt::init();
    trace!("Hello, world!");
}
