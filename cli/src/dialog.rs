use anyhow::Result;
use dialoguer::{Confirm, Input, Select};
use lib::{ChatAssistant, ExpliceConfig};

pub fn confirm_override() -> Result<bool> {
    let confirmation = Confirm::new()
        .with_prompt("Do you want to override?")
        .default(false)
        .interact()?;

    Ok(confirmation)
}

pub fn chat_prompt() -> Result<Option<String>> {
    let input: String = Input::new().allow_empty(true).interact_text()?;
    if input.trim().is_empty() {
        return Ok(None);
    }
    Ok(Some(input))
}

pub fn select_model(models: Vec<String>) -> Result<String> {
    let selected = Select::new()
        .with_prompt("Which model do you want to use?")
        .items(&models)
        .interact()?;

    let model = models.get(selected).unwrap().to_owned();

    Ok(model)
}

pub fn select_assistant(config: &ExpliceConfig) -> Result<&ChatAssistant> {
    let assistants = config.assistants();
    let assistant_names = &assistants.names();

    let selected = Select::new()
        .with_prompt("Which assistant do you want to use?")
        .items(&assistant_names)
        .interact()?;

    let assistant = assistants.get_by_name(assistant_names.get(selected).unwrap())?;

    Ok(assistant)
}
