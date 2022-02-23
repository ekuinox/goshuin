use serenity::{
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args, CommandGroup, CommandOptions, CommandResult, DispatchError, HelpOptions, Reason,
        StandardFramework,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};

#[command]
#[required_permissions("ADMINISTRATOR")]
pub async fn edit(ctx: &Context, msg: &Message) -> CommandResult {
    edit_command(ctx, msg).await
}

pub async fn edit_command(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "edit command unimplemented")
        .await?;
    Ok(())
}
