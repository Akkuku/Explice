use crate::assistant::ChatAssistant;
use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;

pub async fn create_chat_completion(
    apikey: &str,
    prompt: &str,
    assistant: &ChatAssistant,
) -> Result<String> {
    let open_ai_config = OpenAIConfig::new().with_api_key(apikey);
    let client = Client::with_config(open_ai_config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(assistant.model())
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(assistant.system())
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ])
        .max_tokens(40u16)
        .build()?;

    let response = client.chat().create(request).await?;
    let completion = response
        .choices
        .first()
        .expect("model returned no choices")
        .message
        .content
        .as_ref()
        .expect("message content is empty")
        .to_string();

    Ok(completion)
}

pub async fn get_available_chat_models(apikey: &str) -> Result<Vec<String>> {
    let chat_models = get_available_models(apikey)
        .await?
        .into_iter()
        .filter(|name| name.starts_with("gpt"))
        .collect();

    Ok(chat_models)
}

async fn get_available_models(apikey: &str) -> Result<Vec<String>> {
    let open_ai_config = OpenAIConfig::new().with_api_key(apikey);
    let client = Client::with_config(open_ai_config);

    let models = client
        .models()
        .list()
        .await?
        .data
        .iter()
        .map(|model| model.id.to_owned())
        .collect();

    Ok(models)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ExpliceConfig, Persist};

    #[tokio::test]
    async fn test_get_available_chat_models() -> Result<()> {
        let config = ExpliceConfig::read()?;
        let apikey = config.apikey();

        let models = get_available_chat_models(apikey).await?;

        assert!(models.iter().any(|name| name == "gpt-3.5-turbo"));
        assert!(models.iter().any(|name| name == "gpt-4"));

        Ok(())
    }
}
