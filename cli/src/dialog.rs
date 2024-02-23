use crate::completion::PathCompletion;
use anyhow::Result;
use dialoguer::{Input, Select};
use lib::validation::openai_api_key_format_validator;
use lib::{ChatAssistant, ExpliceConfig};

pub fn input_chat_prompt() -> Result<Option<String>> {
    let input: String = Input::new()
        .allow_empty(true)
        .completion_with(&PathCompletion::default())
        .interact_text()?;

    match input.trim().is_empty() {
        true => Ok(None),
        false => Ok(Some(input)),
    }
}

pub fn input_api_key() -> Result<String> {
    let input: String = Input::new()
        .with_prompt("Please input OpenAi API key")
        .validate_with(openai_api_key_format_validator)
        .interact_text()?;

    Ok(input.to_owned())
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
