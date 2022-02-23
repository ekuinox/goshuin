use std::sync::{Arc, Mutex};

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

pub struct Data;

#[derive(Debug)]
pub struct A {
    pub a: String,
}

impl TypeMapKey for Data {
    // While you will be using RwLock or Mutex most of the time you want to modify data,
    // sometimes it's not required; like for example, with static data, or if you are using other
    // kinds of atomic operators.
    //
    // Arc should stay, to allow for the data lock to be closed early.
    type Value = Arc<Mutex<A>>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        {
            let mut data = ctx.data.write().await;
            let data = data.get_mut::<Data>().expect("msg").lock();
            match data {
                Ok(mut data) => {
                    data.a += &message.content;
                    dbg!(&data);
                },
                _ => {

                }
            }
        };

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
