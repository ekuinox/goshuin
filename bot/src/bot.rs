use crate::cli::Cli;
use crate::client::GoshuinRepositoryClient;
use crate::facility::Facility;
use clap::Parser;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};

pub struct Handler {
    admin_id: UserId,
    facility: Option<Facility>,
    client: GoshuinRepositoryClient,
}

impl Handler {
    pub fn new(admin_id: UserId, client: GoshuinRepositoryClient) -> Handler {
        Handler {
            client,
            admin_id,
            facility: None,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.author.id != self.admin_id {
            return;
        }

        let args = message.content.split(' ').collect::<Vec<&str>>();
        if args.get(0).map(|cmd| *cmd != "!goshuin").unwrap_or(true) {
            return;
        }

        dbg!(&args);

        let cli = match Cli::try_parse_from(args) {
            Ok(cli) => cli,
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        };
        match cli {
            Cli::GetSubcommand { id } => {
                if let Ok(facility) = self.client.get_facility(&id).await {
                    let json = match serde_json::to_string_pretty(&facility) {
                        Ok(json) => json,
                        Err(_) => return,
                    };
                    let _ = message.channel_id.say(&ctx.http, json).await;
                } else {
                    let _ = message
                        .channel_id
                        .say(&ctx.http, format!("{} is not found", id))
                        .await;
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
