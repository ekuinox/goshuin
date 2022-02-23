use crate::editor::EditorData;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[owners_only]
pub async fn dump(ctx: &Context, msg: &Message) -> CommandResult {
    dump_command(ctx, msg).await
}

pub async fn dump_command(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let mut editor = data
        .get_mut::<EditorData>()
        .expect("Editor is None")
        .lock()
        .await;
    let facility = editor.get_facility();
    dbg!(&facility);

    let json = serde_json::to_string_pretty(&facility)?;

    let _ = msg.channel_id.say(&ctx.http, json).await?;

    Ok(())
}
