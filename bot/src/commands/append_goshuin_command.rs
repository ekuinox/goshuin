use crate::editor::EditorData;
use clap::Parser;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[owners_only]
pub async fn append_goshuin(ctx: &Context, msg: &Message) -> CommandResult {
    append_goshuin_command(ctx, msg).await
}

#[derive(Parser)]
struct AppendGoshuinArgs {
    pub date: String,
    pub desc: Option<String>,
}

pub async fn append_goshuin_command(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let mut editor = data
        .get_mut::<EditorData>()
        .expect("Editor is None")
        .lock()
        .await;
    let args = msg
        .content
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let args = match AppendGoshuinArgs::try_parse_from(&args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{:#?}", e);
            return Ok(());
        }
    };

    let images = msg.attachments.iter().filter(|attachment| {
        attachment.content_type.as_ref().map(|c| c.starts_with("image")).unwrap_or(false)
    })
    .map(|attachment| {
        (attachment.filename.clone(), attachment.url.clone())
    })
    .collect::<Vec<(String, String)>>();

    for (name, origin) in &images {
        let _ = editor.create_image(origin, name).await?;
    }

    let images = images.into_iter().map(|(name, _)| name).collect::<Vec<String>>();

    let _ = editor.append_goshuin(images, args.date, args.desc)?;

    Ok(())
}
