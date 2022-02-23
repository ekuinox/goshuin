use crate::editor::EditorData;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[required_permissions("ADMINISTRATOR")]
pub async fn save(ctx: &Context, msg: &Message) -> CommandResult {
    save_command(ctx, msg).await
}

pub async fn save_command(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let editor = data
        .get_mut::<EditorData>()
        .expect("Editor is None")
        .lock()
        .await;
    let reply = match editor.write().await {
        Ok(_) => {
            format!("saved")
        },
        Err(e) => {
            let e = format!("{:#?}", e);
            eprintln!("{}", e);
            e
        }
    };
    msg.channel_id.say(&ctx.http, reply).await?;
    Ok(())
}
