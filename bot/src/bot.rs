use crate::client::GoshuinRepositoryClient;
use crate::facility::Facility;
use crate::cli::{Cli, Commands};
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
    async fn message(&self, _ctx: Context, message: Message) {
        if message.author.id != self.admin_id {
            return;
        }

        let args = message.content.split(' ').collect::<Vec<&str>>();
        if args.get(0).map(|cmd| *cmd != "!goshuin").unwrap_or(true) {
            return;
        }
        let args = args.into_iter().skip(1).collect::<Vec<&str>>();

        dbg!(&args);

        let cli = match Cli::try_parse_from(args) {
            Ok(cli) => cli,
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            },
        };
        match cli.command {
            Commands::GetCommand { id } => {
                println!("{}", id);
                let r = self.client.get_facility(id).await;
                println!("{:#?}", r);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
