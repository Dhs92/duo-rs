use serde::{Deserialize, Serialize};
use simplelog;
use std::default::Default;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    token: String,
    log_level: String,
}

impl Config {
    pub fn build(file_name: &str) -> io::Result<Self> {
        let conf_string = fs::read_to_string(file_name)?;

        let conf: Config = serde_json::from_str(&conf_string)?;

        Ok(conf)
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn log_level(&self) -> simplelog::LevelFilter {
        match self.log_level.parse() {
            Ok(ll) => ll,
            Err(_) => simplelog::LevelFilter::Error,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            token: "xxxxxxxxxx".to_string(),
            log_level: "error".to_string(),
        }
    }
}
