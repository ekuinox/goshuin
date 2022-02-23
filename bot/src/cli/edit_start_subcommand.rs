use super::Context;
use anyhow::Result;
use clap::Parser;
use serenity::model::channel::Message;

#[derive(Parser, Debug)]
pub struct EditStartSubcommand {
    id: String,
}

impl EditStartSubcommand {
    pub async fn run(&self, _ctx: &Context, _message: &Message) -> Result<()> {
        Ok(())
    }
}
