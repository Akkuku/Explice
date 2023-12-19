use crate::dialog::{chat_prompt, select_assistant};
use anyhow::Result;
use lib::{
    create_chat_completion, parse_prompt, ChatAssistant, ChatMessageBuilder, ExpliceConfig, Persist,
};

pub async fn chat_cmd(assistant_name: Option<&str>) -> Result<()> {
    let config = ExpliceConfig::read()?;

    let assistant = match assistant_name {
        None => select_assistant(&config)?,
        Some(assistant_name) => config.assistants().get_by_name(assistant_name)?,
    };

    create_chat_loop(&config.apikey(), assistant).await?;

    Ok(())
}

pub async fn create_chat_loop(apikey: &str, assistant: &ChatAssistant) -> Result<()> {
    let mut message_builder = ChatMessageBuilder::new(assistant.system())?;
    loop {
        let prompt = match chat_prompt()? {
            None => {
                break;
            }
            Some(prompt) => parse_prompt(prompt)?,
        };
        message_builder.add_user(&prompt)?;
        let completion =
            create_chat_completion(apikey, assistant, message_builder.build().to_vec()).await?;
        message_builder.add_assistant(&completion)?;

        println!("{completion}");
    }

    Ok(())
}
