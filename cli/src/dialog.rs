use crate::completion::PathCompletion;
use anyhow::Result;
use dialoguer::{BasicHistory, Confirm, Input, Select};
use lib::validation::openai_api_key_format_validator;
use lib::ChatAssistant;

pub fn input_chat_prompt(history: &mut BasicHistory) -> Result<Option<String>> {
    let input: String = Input::new()
        .allow_empty(true)
        .completion_with(&PathCompletion::default())
        .history_with(history)
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

    Ok(input)
}

pub fn select_model(models: Vec<String>) -> Result<String> {
    let selected = Select::new()
        .with_prompt("Which model do you want to use?")
        .items(&models)
        .interact()?;

    let model = models.get(selected).unwrap().to_owned();

    Ok(model)
}

pub fn select_assistant(mut assistants: Vec<ChatAssistant>) -> Result<ChatAssistant> {
    let assistant_names: Vec<_> = assistants
        .iter()
        .map(|assistant| assistant.name())
        .collect();

    let selected = Select::new()
        .with_prompt("Which assistant do you want to use?")
        .items(&assistant_names)
        .interact()?;

    let assistant = assistants.remove(selected);

    Ok(assistant)
}

pub fn confirm_execute() -> Result<bool> {
    let confirmation = Confirm::new()
        .with_prompt("Do you want to execute?")
        .default(true)
        .interact()?;

    Ok(confirmation)
}
