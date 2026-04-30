use crate::config::error::ConfigError;
use crate::config::model::Config;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"
interval_seconds = 60
"#;

pub fn config_path() -> Result<PathBuf, ConfigError> {
    // Get the config directory for the application "./config/file-mover" on linux
    let proj_dirs =
        ProjectDirs::from("com", "Thive-N", "file-mover").ok_or(ConfigError::NoConfigDir)?;

    let config_dir = proj_dirs.config_dir();
    // Create the config directory if it doesn't exist
    fs::create_dir_all(config_dir)?;

    // returns the path to the config file "./config/file-mover/config.toml"
    Ok(config_dir.join("config.toml"))
}

pub fn load_or_create() -> Result<Config, ConfigError> {
    // Get the config file path and create it with default contents if it doesn't exist
    let path = config_path()?;

    if !path.exists() {
        fs::write(&path, DEFAULT_CONFIG)?;
    }

    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), ConfigError> {
    // Get the config file path and write the config to it does not validate the config before saving, caller should validate before calling this function
    let path = config_path()?;
    let contents = toml::to_string_pretty(config)?;
    std::fs::write(path, contents)?;
    Ok(())
}
