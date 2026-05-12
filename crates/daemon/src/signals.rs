use tokio::signal::unix::{SignalKind, signal};
use tracing::info;

pub async fn shutdown_signal() {
    let mut sigterm = signal(SignalKind::terminate()).expect("failed to listen for SIGTERM signal");
    tokio::select! {
        _= tokio::signal::ctrl_c() => {
            info!("CTRL-C received");
        },
        _ = sigterm.recv() => {
            info!("SIGTERM received");
        }
    }
}
