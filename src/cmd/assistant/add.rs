use crate::persistence::assistants::{ChatAssistant, ChatAssistants};

pub(crate) fn add_assistant_cmd(assistant: ChatAssistant) -> anyhow::Result<()> {
    let mut assistants = ChatAssistants::read()?;
    assistants.push(assistant)?;
    assistants.save()?;

    println!("Successfully added assistant");
    Ok(())
}
