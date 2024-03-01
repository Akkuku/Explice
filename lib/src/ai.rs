mod chat;
mod thread;

use crate::ai::chat::Chat;
use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::AssistantObject;
use async_openai::Client;

pub struct OpenAi {
    client: Client<OpenAIConfig>,
}

impl OpenAi {
    pub fn new(api_key: &str) -> Self {
        let open_ai_config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(open_ai_config);

        Self { client }
    }

    pub fn chat(&self) -> Chat {
        Chat::new(&self.client)
    }

    pub async fn assistant_names(&self) -> Result<Vec<String>> {
        let assistants = self.assistants().await?;
        let assistant_names = assistants
            .into_iter()
            .map(|assistant| assistant.name.unwrap_or_default().to_owned())
            .collect();

        Ok(assistant_names)
    }

    pub async fn assistant_by_name(&self, name: &str) -> Result<Option<AssistantObject>> {
        let assistants = self.assistants().await?;
        let assistant = assistants
            .into_iter()
            .find(|assistant| assistant.name.as_deref() == Some(name));

        match assistant {
            None => Ok(None),
            Some(assistant) => Ok(Some(assistant)),
        }
    }

    pub async fn assistants(&self) -> Result<Vec<AssistantObject>> {
        let assistants = self
            .client
            .assistants()
            .list(&vec![("limit", "100")])
            .await?
            .data;
        Ok(assistants)
    }

    pub async fn chat_models(&self) -> Result<Vec<String>> {
        let chat_models = self
            .models()
            .await?
            .into_iter()
            .filter(|name| name.starts_with("gpt"))
            .collect();

        Ok(chat_models)
    }

    async fn models(&self) -> Result<Vec<String>> {
        let models = self.client.models().list().await?.data;
        let model_names = models.iter().map(|model| model.id.to_owned()).collect();

        Ok(model_names)
    }
}
