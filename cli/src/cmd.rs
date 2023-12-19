mod assistant;
mod chat;
mod init;

use crate::cmd::assistant::{match_assistant_cmd, AssistantCommand};
use crate::cmd::chat::chat_cmd;
use crate::cmd::init::init_cmd;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(arg_required_else_help = true)]
    #[command(about = "Initialize config file")]
    Init { apikey: String },
    #[command(about = "Create chat completion")]
    Chat {
        #[arg(long, short)]
        assistant: Option<String>,
    },
    #[command(subcommand)]
    Assistant(AssistantCommand),
}

pub async fn match_cmd(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Init { apikey } => init_cmd(&apikey)?,
        Command::Chat { assistant } => chat_cmd(assistant.as_deref()).await?,
        Command::Assistant(command) => match_assistant_cmd(command).await?,
    }
    Ok(())
}
