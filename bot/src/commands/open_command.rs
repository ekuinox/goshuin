use crate::editor::EditorData;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[owners_only]
pub async fn open(ctx: &Context, msg: &Message) -> CommandResult {
    open_command(ctx, msg).await
}

pub async fn open_command(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let mut editor = data
        .get_mut::<EditorData>()
        .expect("Editor is None")
        .lock()
        .await;
    let id = if let Some(id) = msg.content.split(' ').nth(1) {
        id.to_string()
    } else {
        msg.channel_id.say(&ctx.http, "id not specified").await?;
        return Ok(());
    };
    let reply = if let Ok(facility) = editor.open(&id).await {
        format!("{} is found. name = {}", facility.id, facility.name)
    } else {
        format!("{} is not found.", id)
    };
    msg.channel_id.say(&ctx.http, reply).await?;
    Ok(())
}
