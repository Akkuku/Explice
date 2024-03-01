use async_openai::types::AssistantObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalChatAssistant {
    id: String,
    name: Option<String>,
    model: String,
    system: String,
}

impl ExternalChatAssistant {
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

impl From<AssistantObject> for ExternalChatAssistant {
    fn from(assistant: AssistantObject) -> Self {
        Self {
            id: assistant.id,
            name: assistant.name,
            model: assistant.model,
            system: assistant.instructions.unwrap_or_default(),
        }
    }
}
