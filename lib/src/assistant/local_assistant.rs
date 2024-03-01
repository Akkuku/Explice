use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
