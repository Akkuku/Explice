use crate::dialog::{input_chat_prompt, select_assistant};
use anyhow::Result;
use clap::Args;
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

    create_chat_loop(&config, assistant, input_chat_prompt).await?;

    Ok(())
}
