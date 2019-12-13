use log::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Debug)]
pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, data: Ready) {
        info!("Bot ready! Currently logged in as: {}", data.user.name);

        ctx.set_presence(
            Some(Activity::listening("Jewish Death Metal")),
            OnlineStatus::Online,
        );
    }
}
