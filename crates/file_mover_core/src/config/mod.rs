pub mod editor;
pub mod error;
pub mod loader;
pub mod model;
pub mod validate;

pub use error::ConfigError;
pub use loader::{config_path, load_or_create, save_config};
pub use model::{Config, Rule};
pub use validate::validate_config;
