mod bot;
mod cli;
mod client;
mod facility;

use anyhow::Result;
use dotenv::dotenv;
use octocrab::Octocrab;
use serenity::{model::id::UserId, prelude::*};
use std::{env, str::FromStr};

use crate::{bot::Handler, client::GoshuinRepositoryClient};

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
    let handler = Handler::new(admin_id, client);

    let mut client = Client::builder(&discord_token)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
