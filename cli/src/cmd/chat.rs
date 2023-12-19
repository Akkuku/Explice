use crate::dialog::{chat_prompt, select_assistant};
use anyhow::Result;
use lib::{create_chat_loop, parse_prompt, ExpliceConfig, Persist};

pub(crate) async fn chat_cmd(assistant_name: Option<&str>) -> Result<()> {
    let config = ExpliceConfig::read()?;

    let assistant = match assistant_name {
        None => select_assistant(&config)?,
        Some(assistant_name) => config.assistants().get_by_name(assistant_name)?,
    };

    let create_prompt = || match chat_prompt()? {
        None => Ok(None),
        Some(prompt) => Ok(Some(parse_prompt(prompt)?)),
    };

    create_chat_loop(&config, assistant, create_prompt).await?;

    Ok(())
}
