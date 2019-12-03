mod commands;
mod util;

use lazy_static::*;
use log::*;
use serenity::framework::standard::StandardFramework;
use serenity::Client;
use simplelog::Config as LogConfig;
use simplelog::*;
use std::fs::{write, File, OpenOptions};
use commands::GENERAL_GROUP;
use util::Config;
use util::Handler;

lazy_static! {
    static ref CONFIG: Config = match Config::from_file("config.json") {
        Ok(conf) => conf,
        Err(_) => {
            initialize_logging(false).unwrap();

            let conf = Config::default();

            error!("Config not found, creating default. Add your token to `config.json` and restart the bot!");

            let conf_str = serde_json::to_string_pretty(&conf).unwrap();
            File::create("config.json").unwrap();
            write("config.json", conf_str.as_bytes()).unwrap();

            std::process::exit(4);
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Handler {};
    let mut client = Client::new(CONFIG.token(), handler)?;

    initialize_logging(true)?;

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(";"))
            .group(&GENERAL_GROUP),
    );

    client.start()?;

    Ok(())
}

fn initialize_logging(found: bool) -> Result<(), Box<dyn std::error::Error>> {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("duo.log")?;

    let log_level = if found {
        CONFIG.log_level()
    } else {
        LevelFilter::Info
    };

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Error,
            LogConfig::default(),
            TerminalMode::Mixed,
        )
        .expect("Unable to create TermLogger"),
        WriteLogger::new(log_level, LogConfig::default(), log_file),
    ])?;

    Ok(())

}
