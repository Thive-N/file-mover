use file_mover_core::config::{load_or_create, validate_config};
use file_mover_core::engine::execute_rule;
use std::time::Duration;
use tokio::signal::unix::{SignalKind, signal};
use tracing::{error, info};

async fn shutdown_signal() {
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
/// Main loop of the daemon. Loads config, validates it, executes rules, and sleeps for the configured interval.
/// TODO: Cleaner daemon loop with better error handling and shutdown support.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let config = match load_or_create() {
            Ok(c) => c,
            Err(e) => {
                error!("failed to load config: {}", e);
                tokio::time::sleep(Duration::from_secs(60)).await;
                continue;
            }
        };

        if let Err(errors) = validate_config(&config) {
            for err in errors {
                error!("config validation error: {}", err);
            }

            tokio::time::sleep(Duration::from_secs(60)).await;
            continue;
        }

        info!("running {} rules", config.rules.len());

        for rule in &config.rules {
            info!("executing rule '{}'", rule.name);

            match execute_rule(rule) {
                Ok(result) => {
                    info!("rule '{}' moved {} files", rule.name, result.moved.len());
                }
                Err(e) => {
                    error!("rule '{}' failed: {}", rule.name, e);
                }
            }
        }

        let interval = config.interval_seconds.unwrap_or(60);

        info!("sleeping for {} seconds", interval);

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(interval)) => {

            },
            _ = shutdown_signal() => {
                info!("shutdown signal received, exiting...");
                break;
            }
        }
    }
    info!("daemon exiting");
    Ok(())
}
