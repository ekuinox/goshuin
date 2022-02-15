mod facility;

use anyhow::Result;
use dotenv::dotenv;
use facility::Facility;
use octocrab::{models::repos::Object, params::repos::Reference, Octocrab};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};
use std::{env, str::FromStr};

#[derive(Debug)]
struct Handler {
    admin_id: UserId,
    facility: Option<Facility>,
}

impl Handler {
    pub fn new(admin_id: UserId) -> Handler {
        Handler {
            admin_id,
            facility: None,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, message: Message) {
        if message.author.id != self.admin_id {
            return;
        }
        
        if !message.content.starts_with("!goshuin") {
            return;
        }

        println!("{:#?}", message);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")?;
    let admin_id = env::var("ADMIN_ID")?;
    let admin_id = UserId::from_str(&admin_id)?;

    let mut client = Client::builder(&token)
        .event_handler(Handler::new(admin_id))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let refs = octocrab
        .repos("ekuinox", "goshuin")
        .get_ref(&Reference::Branch("deploy-1".into()))
        .await?;

    let sha = match refs.object {
        Object::Commit { sha, .. } => sha,
        Object::Tag { sha, .. } => sha,
        _ => return Ok(()),
    };

    let content = octocrab
        .repos("ekuinox", "goshuin")
        .get_content()
        .path("facilities/hirose-taisha.json")
        .r#ref(sha)
        .send()
        .await?;

    let item = content.items.first().expect("msg");
    let content = item
        .content
        .to_owned()
        .and_then(|c| {
            // 改行コードが 60 文字区切りで入っているので消していく
            let c = c
                .chars()
                .into_iter()
                .filter(|c| *c != '\n')
                .collect::<String>();
            let decoded = base64::decode(c);
            decoded.ok().and_then(|s| String::from_utf8(s).ok())
        })
        .unwrap_or_default();
    println!("{}", content);

    let facility = serde_json::from_str::<Facility>(&content);

    println!("{:#?}", facility);

    Ok(())
}
