use crate::persistence::assistants::ChatAssistants;

pub(crate) fn list_assistants_cmd() -> anyhow::Result<()> {
    let assistants = ChatAssistants::read()?;
    println!("Available assistants: {:?}", assistants.names());
    Ok(())
}
