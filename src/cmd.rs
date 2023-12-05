mod assistant;
mod chat;
mod init;

use crate::cmd::assistant::{match_assistant_cmd, AssistantCommand};
use crate::cmd::chat::chat_cmd;
use crate::cmd::init::init_cmd;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(arg_required_else_help = true)]
    #[command(about = "Initialize in directory", long_about = None)]
    Init { apikey: String },
    #[command(arg_required_else_help = true)]
    #[command(about = "Create chat completion", long_about = None)]
    Chat {
        #[arg(long, short)]
        assistant: String,
        prompt: String,
    },
    #[command(subcommand)]
    Assistant(AssistantCommand),
}

pub(crate) async fn match_cmd(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Init { apikey } => init_cmd(apikey)?,
        Command::Chat { prompt, assistant } => chat_cmd(prompt, assistant).await?,
        Command::Assistant(command) => match_assistant_cmd(command)?,
    }
    Ok(())
}
