mod runner;
mod signals;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("starting file mover daemon");

    if let Err(e) = runner::run().await {
        error!("daemon crashed: {}", e);
    }
}
