mod assistants;
mod chat;
mod thread;

use anyhow::Result;
use assistants::OpenAiAssistants;
pub use assistants::OpenAiChatAssistant;
use async_openai::config::OpenAIConfig;
use async_openai::Client;
use chat::Chat;
pub use chat::ChatController;

pub struct OpenAi {
    client: Client<OpenAIConfig>,
}

impl OpenAi {
    pub fn new(api_key: &str) -> Self {
        let open_ai_config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(open_ai_config);

        Self { client }
    }

    pub fn chat<C>(&self, controller: C) -> Chat<C>
    where
        C: ChatController,
    {
        Chat::new(&self.client, controller)
    }

    pub fn assistants(&self) -> OpenAiAssistants {
        OpenAiAssistants::new(&self.client)
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
