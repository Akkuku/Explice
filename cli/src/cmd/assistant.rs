mod add;
mod list;

use crate::cmd::assistant::add::{assistant_add_cmd, AssistantAddArgs};
use crate::cmd::assistant::list::assistant_list_cmd;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AssistantCommand {
    #[command(about = "List all assistant names")]
    List,
    #[command(about = "Add assistant")]
    Add(AssistantAddArgs),
}

pub(crate) async fn match_assistant_cmd(command: AssistantCommand) -> anyhow::Result<()> {
    match command {
        AssistantCommand::List => assistant_list_cmd().await?,
        AssistantCommand::Add(args) => assistant_add_cmd(args).await?,
    }
    Ok(())
}
