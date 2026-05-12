use file_mover_core::config::{load_or_create, validate_config};
use file_mover_core::engine::execute_rule;

use std::time::Duration;

use tracing::{error, info};

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

        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}
