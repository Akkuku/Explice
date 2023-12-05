pub(crate) mod add;
pub(crate) mod list;

use crate::cmd::assistant::add::add_assistant_cmd;
use crate::cmd::assistant::list::list_assistants_cmd;
use crate::persistence::assistants::ChatAssistant;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub(crate) enum AssistantCommand {
    #[command(about = "List all assistant names", long_about = None)]
    List,
    #[command(about = "Add assistant", long_about = None)]
    Add(ChatAssistant),
}

pub(crate) fn match_assistant_cmd(command: AssistantCommand) -> anyhow::Result<()> {
    match command {
        AssistantCommand::List => list_assistants_cmd()?,
        AssistantCommand::Add(assistant) => add_assistant_cmd(assistant)?,
    }
    Ok(())
}
