use crate::assistant::ChatAssistant;
use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;

pub async fn create_chat_completion(
    apikey: &str,
    assistant: &ChatAssistant,
    messages: Vec<ChatCompletionRequestMessage>,
) -> Result<String> {
    let open_ai_config = OpenAIConfig::new().with_api_key(apikey);
    let client = Client::with_config(open_ai_config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(assistant.model())
        .messages(messages)
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

pub struct ChatMessageBuilder {
    messages: Vec<ChatCompletionRequestMessage>,
}

impl ChatMessageBuilder {
    pub fn new(system_message: &str) -> Result<Self> {
        Ok(Self {
            messages: vec![ChatCompletionRequestSystemMessageArgs::default()
                .content(system_message)
                .build()?
                .into()],
        })
    }

    pub fn add_user(&mut self, prompt: &str) -> Result<&mut Self> {
        self.messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );
        Ok(self)
    }

    pub fn add_assistant(&mut self, completion: &str) -> Result<&mut Self> {
        self.messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(completion)
                .build()?
                .into(),
        );
        Ok(self)
    }

    pub fn build(&self) -> &Vec<ChatCompletionRequestMessage> {
        &self.messages
    }
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
