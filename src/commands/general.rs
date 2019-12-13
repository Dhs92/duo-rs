use serenity::client::Context;

use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    http::CacheHttp,
    model::prelude::*,
    voice, Result as SerenityResult,
};

use crate::VoiceManager;

use log::{debug, error, warn};

group!({
    name: "general",
    options: {},
    commands: [ping, rick_roll],
});

// TODO: create fun commands

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong!")?;

    Ok(())
}

#[command]
pub fn rick_roll(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("Entering rick_roll");

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Groups and DMs not supported"),
            );

            return Ok(());
        }
    };

    let channel_id = args.single::<ChannelId>()?;

    debug!("channel_id: {}", channel_id);

    let manager_lock = ctx
        .data
        .read()
        .get::<VoiceManager>()
        .cloned()
        .expect("Expected VoiceManager in ShareMap.");

    let mut manager = manager_lock.lock();

    if manager.join(guild_id, channel_id).is_some() {
        debug!(
            "Joined {} successfully",
            channel_id.name(&ctx.cache().unwrap()).unwrap()
        )
    } else {
        error!(
            "Could not join {}",
            channel_id.name(&ctx.cache().unwrap()).unwrap()
        )
    }

    if let Some(handler) = manager.get_mut(guild_id) {
        let source = match voice::ytdl("https://www.youtube.com/watch?v=MU0Yp0qmYEs") {
            Ok(source) => source,
            Err(why) => {
                error!("Err starting source: {:?}", why);

                return Ok(());
            }
        };

        debug!("Playing song... {:?}", handler);
        handler.play(source);
        debug!("Past handler.play()")
    }

    Ok(())
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        warn!("Error sending message: {:?}", why);
    }
}

// TODO: come up with design for this
// #[command]
// pub fn clean(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {

// }
