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

use crate::editor::EditorData;
use super::get_args;

#[command]
#[required_permissions("ADMINISTRATOR")]
pub async fn open(ctx: &Context, msg: &Message) -> CommandResult {
    open_command(ctx, msg).await
}

pub async fn open_command(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let mut editor = data.get_mut::<EditorData>().expect("Editor is None").lock().await;
    let args = get_args(&msg.content);
    let reply= if let Ok(facility) = editor.open(&args[0]).await {
        format!("{} is found. name = {}", facility.id, facility.name)
    } else {
        format!("{} is not found.", args[0])
    };
    msg.channel_id
        .say(&ctx.http, reply)
        .await?;
    Ok(())
}
