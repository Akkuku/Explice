mod complete;
mod init;

use crate::cmd::complete::chat_cmd;
use crate::cmd::init::init_cmd;
use crate::Cli;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(arg_required_else_help = true)]
    #[command(about = "Initialize in directory", long_about = None)]
    Init { apikey: String },
    #[command(arg_required_else_help = true)]
    #[command(about = "Create chat completion", long_about = None)]
    Chat { prompt: String },
}

pub(crate) async fn match_commands(args: Cli) {
    match args.command {
        Commands::Init { apikey } => init_cmd(apikey),
        Commands::Chat { prompt } => chat_cmd(prompt).await,
    }
}
