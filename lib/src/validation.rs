use crate::get_available_chat_models;
use anyhow::{anyhow, bail};

pub fn openai_api_key_format_validator(api_key: &String) -> anyhow::Result<()> {
    let re = regex::Regex::new(r"^sk-[0-9a-f]{32}$").unwrap();
    if !re.is_match(api_key) {
        bail!("Invalid format for OpenAi API key")
    }

    Ok(())
}

pub async fn openai_api_key_request_validator(api_key: &str) -> anyhow::Result<()> {
    get_available_chat_models(api_key)
        .await
        .map_err(|_| anyhow!("Failed to send API request with provided API key"))?;

    Ok(())
}
