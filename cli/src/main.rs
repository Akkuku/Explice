mod cmd;
mod dialog;

use crate::cmd::{match_cmd, Command};
use clap::Parser;
use lib::APP_NAME;

#[derive(Debug, Parser)]
#[command(name = APP_NAME, about = "Easy AI files workflow")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if let Err(err) = match_cmd(args.command).await {
        eprintln!("{err}")
    };
}
