mod assistant;
mod chat;
mod config;
mod shell;

use crate::cmd::assistant::{match_assistant_cmd, AssistantCommand};
use crate::cmd::chat::{chat_cmd, ChatArgs};
use crate::cmd::config::{config_cmd, ConfigArgs};
use crate::cmd::shell::{shell_cmd, ShellArgs};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    #[command(about = "Manage assistants")]
    Assistant(AssistantCommand),
    #[command(about = "Create chat completion")]
    Chat(ChatArgs),
    #[command(about = "Initialize or update config file")]
    Config(ConfigArgs),
    #[command(name = "sh", about = "Execute shell command")]
    Shell(ShellArgs),
}

pub async fn match_cmd(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Assistant(command) => match_assistant_cmd(command).await?,
        Command::Chat(args) => chat_cmd(args).await?,
        Command::Config(args) => config_cmd(args).await?,
        Command::Shell(args) => shell_cmd(args).await?,
    }
    Ok(())
}
