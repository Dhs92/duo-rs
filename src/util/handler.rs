use log::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Debug)]
pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, data: Ready) {
        info!("Bot ready! Currently logged in as: {}", data.user.name)
    }
}
