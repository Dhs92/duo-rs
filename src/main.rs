mod commands;
mod util;

use lazy_static::*;
use log::*;
use serenity::framework::standard::StandardFramework;
use serenity::Client;
use simplelog::Config as LogConfig;
use simplelog::*;
use std::fs::{write, File, OpenOptions};
// use std::collections::HashMap;
use commands::GENERAL_GROUP;
use util::Config;
use util::Handler;

lazy_static! {
    static ref CONFIG: Config = match Config::build("config.json") {
        Ok(conf) => conf,
        Err(_) => {
            let conf = Config::default();

            let conf_str = serde_json::to_string_pretty(&conf).unwrap();
            File::create("config.json").unwrap();
            write("config.json", conf_str.as_bytes()).unwrap();

            conf
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Handler {};
    let mut client = Client::new(CONFIG.token(), handler)?;

    initialize_logging()?;

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(";"))
            .group(&GENERAL_GROUP),
    );

    client.start()?;

    Ok(())
}

fn initialize_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("duo.log")?;

    CombinedLogger::init(vec![
        TermLogger::new(
            CONFIG.log_level(),
            LogConfig::default(),
            TerminalMode::Mixed,
        )
        .expect("Unable to create TermLogger"),
        WriteLogger::new(LevelFilter::Info, LogConfig::default(), log_file),
    ])?;

    Ok(())
}
