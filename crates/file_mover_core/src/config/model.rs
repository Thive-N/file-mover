use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub interval_seconds: Option<u64>,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub folder: PathBuf,
    pub destination: PathBuf,
    #[serde(default)]
    pub whitelist: Vec<String>,
    #[serde(default)]
    pub blacklist: Vec<String>,
    #[serde(default)]
    pub extensions: Vec<String>,
}
