mod chat_controller;
mod cmd;
mod completion;
mod dialog;
mod storage;

use crate::cmd::{match_cmd, Command};
use clap::Parser;
use lib::APP_NAME;

#[derive(Debug, Parser)]
#[command(name = APP_NAME, about = "Command line AI assistant")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
pub async fn main() {
    let args = Cli::parse();
    if let Err(err) = match_cmd(args.command).await {
        eprintln!("{err:?}")
    };
}
