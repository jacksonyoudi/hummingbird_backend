use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub websocket: WsServerConfig,
}

impl Config {
    pub fn load(filename: impl AsRef<Path>) -> Self {
        let content = fs::read_to_string(filename).unwrap();
        serde_yaml::from_str(&content).unwrap()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WsServerConfig {
    pub protocal: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub tags: Vec<String>,
}
