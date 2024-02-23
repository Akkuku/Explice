mod assistant;
mod chat;
mod config;

use crate::cmd::assistant::{match_assistant_cmd, AssistantCommand};
use crate::cmd::chat::{chat_cmd, ChatArgs};
use crate::cmd::config::{config_cmd, ConfigArgs};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Initialize or update config file")]
    Config(ConfigArgs),
    #[command(about = "Create chat completion")]
    Chat(ChatArgs),
    #[command(subcommand)]
    #[command(about = "Manage assistants")]
    Assistant(AssistantCommand),
}

pub async fn match_cmd(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Config(args) => config_cmd(args).await?,
        Command::Chat(args) => chat_cmd(args).await?,
        Command::Assistant(command) => match_assistant_cmd(command).await?,
    }
    Ok(())
}
