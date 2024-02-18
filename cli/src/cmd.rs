mod assistant;
mod chat;
mod config;
mod init;

use crate::cmd::assistant::{match_assistant_cmd, AssistantCommand};
use crate::cmd::chat::{chat_cmd, ChatArgs};
use crate::cmd::config::{config_cmd, ConfigArgs};
use crate::cmd::init::{init_cmd, InitArgs};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Initialize config file")]
    Init(InitArgs),
    #[command(about = "Update config")]
    Config(ConfigArgs),
    #[command(about = "Create chat completion")]
    Chat(ChatArgs),
    #[command(subcommand)]
    Assistant(AssistantCommand),
}

pub async fn match_cmd(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Init(args) => init_cmd(args)?,
        Command::Config(args) => config_cmd(args)?,
        Command::Chat(args) => chat_cmd(args).await?,
        Command::Assistant(command) => match_assistant_cmd(command).await?,
    }
    Ok(())
}
