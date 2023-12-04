mod complete;
pub mod init;

use crate::cmd::complete::complete_cmd;
use crate::cmd::init::init_cmd;
use crate::Cli;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(arg_required_else_help = true)]
    #[command(about = "Initialize in directory", long_about = None)]
    Init { apikey: String },
    #[command(arg_required_else_help = true)]
    #[command(about = "Initialize in directory", long_about = None)]
    Complete { prompt: String },
}

pub(crate) fn match_commands(args: Cli) {
    match args.command {
        Commands::Init { apikey } => init_cmd(apikey),
        Commands::Complete { prompt } => complete_cmd(prompt),
    }
}
