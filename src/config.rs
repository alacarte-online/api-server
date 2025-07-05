mod database;

use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub image_folder: PathBuf,
    pub database: database::DatabaseConfig,
    pub log_level: log::Level,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigFile {
    pub address: Option<String>,
    pub image_folder: Option<PathBuf>,
    pub database: database::ConfigFileDatabaseTable,
    pub verbose: Option<log::Level>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ValueOrPath {
    #[serde(rename = "value")]
    Value(String),
    #[serde(rename = "path")]
    Path(PathBuf)
}

impl ValueOrPath {
    pub fn try_convert_to_value(self) -> anyhow::Result<String> {
        let value = match self {
            ValueOrPath::Value(value) => Ok(value),
            ValueOrPath::Path(path) => fs::read_to_string(&path)
        }?;
        Ok(value)
    }
}