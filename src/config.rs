use std::time;
use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub servers: Vec<Server>,
    pub interval: time::Duration,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Server {
    pub name: String,
    pub endpoint: String,
    pub timeout: time::Duration,
}

pub fn load_config(config_path: &str) -> Result<Config, serde_yaml::Error> {
    let cfg_file = File::open(config_path).unwrap();
    let cfg: Config = serde_yaml::from_reader(cfg_file)?;
    Ok(cfg)
}

