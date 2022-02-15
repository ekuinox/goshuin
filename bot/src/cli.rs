use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub enum Cli {
    #[clap(name = "get")]
    GetSubcommand {
        id: String,
    },
}
