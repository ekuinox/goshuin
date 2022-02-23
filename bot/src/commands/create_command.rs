use crate::{
    editor::EditorData,
    facility::{Coordinate, FacilityKind},
};
use clap::Parser;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[required_permissions("ADMINISTRATOR")]
pub async fn create(ctx: &Context, msg: &Message) -> CommandResult {
    create_command(ctx, msg).await
}

#[derive(Parser, Debug)]
struct CreateArgs {
    pub id: String,

    pub name: String,

    pub kind: FacilityKind,

    pub lat: f64,

    pub lon: f64,
}

pub async fn create_command(ctx: &Context, msg: &Message) -> CommandResult {
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
    let args = CreateArgs::try_parse_from(&args)?;

    let facility = editor.create(
        args.id,
        args.name,
        args.kind,
        Coordinate::new(args.lat, args.lon),
    );

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "created with id = {}, name = {}, kind = {:?}, lat = {}, lon = {}",
                facility.id,
                facility.name,
                facility.kind,
                facility.coordinate.lat,
                facility.coordinate.lon
            ),
        )
        .await?;
    Ok(())
}
