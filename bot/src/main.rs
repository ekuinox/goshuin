mod bot;
mod cli;
mod client;
mod facility;

use anyhow::Result;
use bot::{Data, A};
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
    sync::{Arc, Mutex}, collections::HashSet,
};

use crate::{bot::Handler, client::GoshuinRepositoryClient};

#[group]
#[commands(test, test1)]
struct General;

#[command]
#[required_permissions("ADMINISTRATOR")]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "This is a small test-bot! : )")
        .await?;

    Ok(())
}

#[command]
#[required_permissions("ADMINISTRATOR")]
async fn test1(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "This is a small test1-bot! : )")
        .await?;

    Ok(())
}

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
        .event_handler(handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Data>(Arc::new(Mutex::new(A { a: "".to_string() })));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
