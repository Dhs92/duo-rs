use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::*,
    prelude::*,
};

group!({
    name: "general",
    options: {},
    commands: [ping],
});

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong!")?;

    Ok(())
}

// #[command]
// pub fn clean(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {

// }
