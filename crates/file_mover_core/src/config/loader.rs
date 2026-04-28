use crate::config::error::ConfigError;
use crate::config::model::Config;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"
interval_seconds = 60
"#;

pub fn config_path() -> Result<PathBuf, ConfigError> {
    let proj_dirs =
        ProjectDirs::from("com", "Thive-N", "file-mover").ok_or(ConfigError::NoConfigDir)?;

    let config_dir = proj_dirs.config_dir();

    fs::create_dir_all(config_dir)?;

    Ok(config_dir.join("config.toml"))
}

pub fn load_or_create() -> Result<Config, ConfigError> {
    let path = config_path()?;

    if !path.exists() {
        fs::write(&path, DEFAULT_CONFIG)?;
    }

    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), ConfigError> {
    let path = config_path()?;
    let contents = toml::to_string_pretty(config)?;
    std::fs::write(path, contents)?;
    Ok(())
}
