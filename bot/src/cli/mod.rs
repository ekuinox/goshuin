mod edit_start_subcommand;
mod context;

use anyhow::Result;
use clap::Parser;
use serenity::model::channel::Message;
pub use edit_start_subcommand::EditStartSubcommand;
pub use context::Context;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub enum Cli {
    #[clap(name = "get")]
    GetSubcommand { id: String },
    #[clap(name = "edit-start")]
    EditStartSubcommand(EditStartSubcommand),
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
