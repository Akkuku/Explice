use crate::dialog::{input_chat_prompt, select_assistant};
use anyhow::Result;
use clap::Args;
use dialoguer::BasicHistory;
use lib::{create_chat_loop, ExpliceConfig};

#[derive(Debug, Args)]
pub struct ChatArgs {
    #[arg(long = "assistant", short)]
    assistant_name: Option<String>,
}

pub(crate) async fn chat_cmd(args: ChatArgs) -> Result<()> {
    let config = ExpliceConfig::read()?;

    let assistant = match args.assistant_name {
        None => select_assistant(&config)?,
        Some(assistant_name) => config.assistants().get_by_name(&assistant_name)?,
    };

    let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);
    let create_prompt = move || input_chat_prompt(&mut history);
    create_chat_loop(&config, assistant, create_prompt).await?;

    Ok(())
}
