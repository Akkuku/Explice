mod add;
mod list;

use crate::cmd::assistant::add::assistant_add_cmd;
use crate::cmd::assistant::list::assistant_list_cmd;
use clap::{Args, Subcommand};
use lib::AssistantData;

#[derive(Debug, Subcommand)]
pub enum AssistantCommand {
    #[command(about = "List all assistant names")]
    List,
    #[command(about = "Add assistant")]
    Add(Assistant),
}

#[derive(Debug, Args)]
pub struct Assistant {
    #[arg(long, short)]
    name: String,
    #[arg(long, short)]
    model: Option<String>,
    #[arg(long, short, default_value = "")]
    description: String,
    #[arg(long, short, default_value = "You are a helpful assistant")]
    system: String,
}

pub(crate) async fn match_assistant_cmd(command: AssistantCommand) -> anyhow::Result<()> {
    match command {
        AssistantCommand::List => assistant_list_cmd()?,
        AssistantCommand::Add(assistant) => assistant_add_cmd(assistant).await?,
    }
    Ok(())
}

impl From<Assistant> for AssistantData {
    fn from(assistant: Assistant) -> Self {
        Self {
            name: assistant.name,
            model: assistant.model.unwrap(),
            description: assistant.description,
            system: assistant.system,
        }
    }
}
