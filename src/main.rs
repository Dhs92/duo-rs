mod commands;
mod util;

use commands::GENERAL_GROUP;
use log::*;
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::prelude::*;
use serenity::Client;
use serenity::{client::Context, model::prelude::*, prelude::Mutex};
use simplelog::Config as LogConfig;
use simplelog::*;
use std::fs::{write, File, OpenOptions};
use std::sync::Arc;
use util::Config;
use util::Handler;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match Config::from_file("config.json") {
        Ok(conf) => conf,
        Err(_) => {
            initialize_logging(LevelFilter::Error)?;

            let conf = Config::default();

            error!("Config not found, creating default. Add your token to `config.json` and restart the bot!");

            #[allow(unreachable_code)]
            let conf_str = match serde_json::to_string_pretty(&conf) {
                Ok(conf_string) => conf_string,
                Err(e) => {
                    error!("{}", e);
                    std::process::exit(6)
                }
            };
            File::create("config.json")?;
            write("config.json", conf_str.as_bytes())?;

            std::process::exit(4);
        }
    };

    let handler = Handler {};
    let mut client = Client::new(config.token(), handler)?;

    initialize_logging(config.log_level())?;

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(";"))
            .group(&GENERAL_GROUP)
            .after(err_handler),
    );

    client.start()?;

    Ok(())
}

fn initialize_logging(log_level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("duo.log")?;

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

fn err_handler(_ctx: &mut Context, _msg: &Message, _s: &str, result: CommandResult) {
    match result {
        Ok(_) => (),
        Err(e) => error!("{:?}", e),
    }
}
