mod bot;
mod cli;
mod client;
mod commands;
mod editor;
mod facility;

use anyhow::Result;
use dotenv::dotenv;
use octocrab::Octocrab;
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
use std::{
    env,
    str::FromStr,
    sync::Arc, collections::HashSet,
};

use crate::client::GoshuinRepositoryClient;
use crate::commands::*;
use crate::editor::{Editor, EditorData};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let discord_token = env::var("DISCORD_TOKEN")?;
    let admin_id = env::var("ADMIN_ID")?;
    let admin_id = UserId::from_str(&admin_id)?;
    let github_token = std::env::var("GITHUB_TOKEN")?;
    let octocrab = Octocrab::builder().personal_token(github_token).build()?;
    let client = GoshuinRepositoryClient::new(
        octocrab,
        "ekuinox".to_string(),
        "goshuin".to_string(),
        "deploy".to_string(),
    );
    let editor = Editor::new(client);
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(None)
                .prefix("~")
                // In this case, if "," would be first, a message would never
                // be delimited at ", ", forcing you to trim your arguments if you
                // want to avoid whitespaces at the start of each.
                .delimiters(vec![", ", ","])
                // Sets the bot's owners. These will be used for commands that
                // are owners only.
                .owners(HashSet::from_iter(vec![admin_id].into_iter()))
        })
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&discord_token)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<EditorData>(Arc::new(Mutex::new(editor)));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
