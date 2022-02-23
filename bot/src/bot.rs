use crate::cli::{Cli, Context as CliContext};
use crate::client::GoshuinRepositoryClient;
use clap::Parser;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};

pub struct Handler {
    admin_id: UserId,
    context: CliContext,
}

impl Handler {
    pub fn new(admin_id: UserId, client: GoshuinRepositoryClient) -> Handler {
        Handler {
            admin_id,
            context: CliContext::new(client),
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

        dbg!(&args);

        let cli = match Cli::try_parse_from(args) {
            Ok(cli) => cli,
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        };
        if let Err(e) = cli.run(&self.context, &message).await {
            eprintln!("{:#?}", e);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
