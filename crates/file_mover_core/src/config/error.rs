use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to determine config directory")]
    NoConfigDir,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("toml serialize error: {0}")]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("config must contain at least one rule")]
    NoRules,

    #[error("interval_seconds must be greater than 0")]
    InvalidInterval,

    #[error("duplicate rule name: {0}")]
    DuplicateRuleName(String),

    #[error("rule '{rule}' has an empty name")]
    EmptyRuleName { rule: String },

    #[error("rule '{rule}' folder does not exist: {path:?}")]
    FolderMissing { rule: String, path: PathBuf },

    #[error("rule '{rule}' folder is not a directory: {path:?}")]
    FolderNotDirectory { rule: String, path: PathBuf },

    #[error("rule '{rule}' must have at least one filter")]
    NoFilters { rule: String },

    #[error("rule '{rule}' has invalid extension: {extension}")]
    InvalidExtension { rule: String, extension: String },

    #[error("rule '{rule}' has invalid glob pattern: {pattern}")]
    InvalidGlob { rule: String, pattern: String },
}
