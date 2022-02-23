use crate::editor::EditorData;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[required_permissions("ADMINISTRATOR")]
pub async fn append_goshuin(ctx: &Context, msg: &Message) -> CommandResult {
    append_goshuin_command(ctx, msg).await
}

pub async fn append_goshuin_command(ctx: &Context, msg: &Message) -> CommandResult {
    todo!("impl");
}
