use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatAssistants {
    assistants: Vec<ChatAssistant>,
}

impl Default for ChatAssistants {
    fn default() -> Self {
        let assistant = ChatAssistant {
            name: "assistant".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            description: "default assistant".to_string(),
            system: "You are a helpful assistant".to_string(),
        };
        Self {
            assistants: vec![assistant],
        }
    }
}

impl ChatAssistants {
    pub fn push(&mut self, assistant: ChatAssistant) -> Result<&Self> {
        if self.assistants.iter().any(|a| a.name == assistant.name) {
            bail!("Assistant with name {:?} already exists", assistant.name);
        }
        self.assistants.push(assistant);

        Ok(self)
    }

    pub fn names(&self) -> Vec<String> {
        self.assistants
            .iter()
            .map(|assistant| assistant.name.to_owned())
            .collect()
    }

    pub fn get_by_name(&self, assistant_name: &str) -> Result<&ChatAssistant> {
        self.assistants
            .iter()
            .find(|assistant| assistant.name == assistant_name)
            .with_context(|| format!("Assistant {} not found", &assistant_name))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatAssistant {
    name: String,
    model: String,
    description: String,
    system: String,
}

impl ChatAssistant {
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
    pub description: String,
    pub system: String,
}

impl From<AssistantData> for ChatAssistant {
    fn from(assistant: AssistantData) -> Self {
        Self {
            name: assistant.name,
            model: assistant.model,
            description: assistant.description,
            system: assistant.system,
        }
    }
}
