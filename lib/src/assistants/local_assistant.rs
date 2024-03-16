use crate::{ChatAssistant, KVStorage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalChatAssistant {
    name: String,
    model: String,
    system: String,
}

impl Default for LocalChatAssistant {
    fn default() -> Self {
        Self {
            name: "assistant".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            system: "You are a helpful assistant".to_string(),
        }
    }
}

impl LocalChatAssistant {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn model(&self) -> &str {
        &self.model
    }
    pub fn system(&self) -> &str {
        &self.system
    }
}

pub struct AssistantData {
    pub name: String,
    pub model: String,
    pub system: String,
}

impl From<AssistantData> for LocalChatAssistant {
    fn from(assistant: AssistantData) -> Self {
        Self {
            name: assistant.name,
            model: assistant.model,
            system: assistant.system,
        }
    }
}

pub struct LocalAssistants<S>
where
    S: KVStorage<String, LocalChatAssistant>,
{
    storage: S,
}

impl<S> LocalAssistants<S>
where
    S: KVStorage<String, LocalChatAssistant>,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn init(&self) -> anyhow::Result<()> {
        let default_assistant = LocalChatAssistant::default();
        self.storage
            .add(default_assistant.name.to_owned(), default_assistant)?;

        Ok(())
    }

    pub fn list(&self) -> anyhow::Result<Vec<ChatAssistant>> {
        let local_assistants: Vec<LocalChatAssistant> = self.storage.get_all()?;
        let assistants = local_assistants
            .into_iter()
            .map(ChatAssistant::LocalAssistant)
            .collect();

        Ok(assistants)
    }

    pub fn names(&self) -> anyhow::Result<Vec<String>> {
        let local_assistants: Vec<LocalChatAssistant> = self.storage.get_all()?;
        let assistant_names = local_assistants
            .into_iter()
            .map(|a| a.name().to_owned())
            .collect();

        Ok(assistant_names)
    }

    pub fn add(&self, assistant: LocalChatAssistant) -> anyhow::Result<()> {
        self.storage.add(assistant.name.to_owned(), assistant)
    }
}
