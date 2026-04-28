use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub interval_seconds: Option<u64>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub folder: PathBuf,
    pub destination: PathBuf,
    pub whitelist: Option<Vec<String>>,
    pub blacklist: Option<Vec<String>>,
    pub extensions: Option<Vec<String>>,
}
