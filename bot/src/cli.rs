use crate::client::GoshuinRepositoryClient;
use anyhow::Result;
use clap::Parser;
use serenity::model::channel::Message;

#[derive(Parser, Debug)]
pub struct EditStartSubcommand {
    id: String,
}

impl EditStartSubcommand {
    async fn run(&self, _ctx: &Context, _message: &Message) -> Result<()> {
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub enum Cli {
    #[clap(name = "get")]
    GetSubcommand { id: String },
    #[clap(name = "edit-start")]
    EditStartSubcommand(EditStartSubcommand),
}

pub struct Context {
    client: GoshuinRepositoryClient,
}

impl Context {
    pub fn new(client: GoshuinRepositoryClient) -> Context {
        Context { client }
    }
}

impl Cli {
    pub async fn run(&self, ctx: &Context, message: &Message) -> Result<()> {
        let r = match self {
            Cli::EditStartSubcommand(s) => s.run(ctx, message).await?,
            _ => {}
        };
        Ok(r)
    }
}
