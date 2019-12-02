use serde::{Deserialize, Serialize, Deserializer, Serializer};
use simplelog;
use std::default::Default;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    token: String,

    #[serde(with = "serde_level_filter")]
    log_level: simplelog::LevelFilter,
}

impl Config {
    pub fn from_file(file_name: &str) -> io::Result<Self> {
        let conf_string = fs::read_to_string(file_name)?;

        let conf: Config = serde_json::from_str(&conf_string)?;

        Ok(conf)
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn log_level(&self) -> simplelog::LevelFilter {
        self.log_level
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            token: "xxxxxxxxxx".to_string(),
            log_level: simplelog::LevelFilter::Error,
        }
    }
}

// this will allow de/serialization of LevelFilter
pub mod serde_level_filter {
    use super::*;
    
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(lf: &simplelog::LevelFilter, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        lf.to_string().serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<simplelog::LevelFilter, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let filter = string.parse().map_err(serde::de::Error::custom)?;
        Ok(filter)
    }

}
