mod cmd;

use crate::cmd::{match_commands, Commands};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "explice")]
#[command(about = "Easy AI files workflow", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
fn main() {
    let args = Cli::parse();
    match_commands(args);
}
