mod cmd;
mod persistence;
mod use_case;

use crate::cmd::{match_cmd, Command};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "explice")]
#[command(about = "Easy AI files workflow", long_about = None)]
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
