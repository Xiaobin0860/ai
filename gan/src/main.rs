use gan::{App, AppArgs, AppResult};

use tracing::trace;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt::init();
    let args = AppArgs::default();
    trace!("{args:?}");
    App::new(&args.cfg, args.clean)?.run().await
}
