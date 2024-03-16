use crate::ChatAssistant;
use async_openai::config::OpenAIConfig;
use async_openai::types::AssistantObject;
use async_openai::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiChatAssistant {
    id: String,
    name: Option<String>,
    model: String,
    system: String,
}

impl OpenAiChatAssistant {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.id)
    }
    pub fn model(&self) -> &str {
        &self.model
    }
    pub fn system(&self) -> &str {
        &self.system
    }
}

impl From<AssistantObject> for OpenAiChatAssistant {
    fn from(assistant: AssistantObject) -> Self {
        Self {
            id: assistant.id,
            name: assistant.name,
            model: assistant.model,
            system: assistant.instructions.unwrap_or_default(),
        }
    }
}

pub struct OpenAiAssistants<'c> {
    client: &'c Client<OpenAIConfig>,
}

impl<'c> OpenAiAssistants<'c> {
    pub(crate) fn new(open_ai_client: &'c Client<OpenAIConfig>) -> Self {
        Self {
            client: open_ai_client,
        }
    }

    pub async fn list(&self) -> anyhow::Result<Vec<ChatAssistant>> {
        let assistants = self
            .assistants()
            .await?
            .into_iter()
            .map(OpenAiChatAssistant::from)
            .map(ChatAssistant::ExternalAssistant)
            .collect();

        Ok(assistants)
    }

    pub async fn names(&self) -> anyhow::Result<Vec<String>> {
        let assistants: Vec<_> = self.list().await?;
        let assistant_names = assistants
            .into_iter()
            .map(|a| a.name().to_owned())
            .collect();

        Ok(assistant_names)
    }

    async fn assistants(&self) -> anyhow::Result<Vec<AssistantObject>> {
        let assistants = self
            .client
            .assistants()
            .list(&vec![("limit", "100")])
            .await?
            .data;
        Ok(assistants)
    }
}
