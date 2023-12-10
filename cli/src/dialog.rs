use anyhow::Result;
use dialoguer::{Confirm, Select};

pub fn confirm_override() -> Result<bool> {
    let confirmation = Confirm::new()
        .with_prompt("Do you want to override?")
        .default(false)
        .interact()?;

    Ok(confirmation)
}

pub async fn select_model(models: Vec<String>) -> Result<String> {
    let selected = Select::new()
        .with_prompt("Which model do you want to use?")
        .items(&models)
        .interact()?;

    let model = models.get(selected).unwrap().to_owned();

    Ok(model)
}

pub fn select_assistant(assistants: Vec<String>) -> Result<String> {
    let selected = Select::new()
        .with_prompt("Which assistant do you want to use?")
        .items(&assistants)
        .interact()?;

    let assistant = assistants.get(selected).unwrap().to_owned();

    Ok(assistant)
}
