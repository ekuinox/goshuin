mod client;
mod commands;
mod editor;
mod facility;

use anyhow::Result;
use dotenv::dotenv;
use octocrab::Octocrab;
use serenity::{framework::standard::StandardFramework, model::id::UserId, prelude::*};
use std::{collections::HashSet, env, str::FromStr, sync::Arc};

use crate::client::GoshuinRepositoryClient;
use crate::commands::*;
use crate::editor::{Editor, EditorData};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let github_repository_owner = env::var("GITHUB_REPOSITORY_OWNER")?;
    let github_repository_name = env::var("GITHUB_REPOSITORY_NAME")?;
    let github_deploy_branch_name = env::var("GITHUB_DEPLOY_BRANCH_NAME")?;
    let discord_token = env::var("DISCORD_TOKEN")?;
    let admin_id = env::var("ADMIN_ID")?;
    let admin_id = UserId::from_str(&admin_id)?;
    let github_token = std::env::var("GITHUB_TOKEN")?;
    let octocrab = Octocrab::builder().personal_token(github_token).build()?;
    let client = GoshuinRepositoryClient::new(
        octocrab,
        github_repository_owner,
        github_repository_name,
        github_deploy_branch_name,
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
