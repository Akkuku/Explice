mod external_assistant;
mod local_assistant;

pub use external_assistant::*;
pub use local_assistant::*;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatAssistants {
    assistants: Vec<LocalChatAssistant>,
}

impl Default for ChatAssistants {
    fn default() -> Self {
        Self {
            assistants: vec![LocalChatAssistant::default()],
        }
    }
}

impl ChatAssistants {
    pub fn push(&mut self, assistant: LocalChatAssistant) -> Result<&Self> {
        if self.assistants.iter().any(|a| a.name() == assistant.name()) {
            bail!("Assistant with name {:?} already exists", assistant.name());
        }
        self.assistants.push(assistant);

        Ok(self)
    }

    pub fn names(&self) -> Vec<String> {
        self.assistants
            .iter()
            .map(|assistant| assistant.name().to_owned())
            .collect()
    }

    pub fn get_by_name(&self, assistant_name: &str) -> Result<&LocalChatAssistant> {
        self.assistants
            .iter()
            .find(|assistant| assistant.name() == assistant_name)
            .with_context(|| format!("Assistant {} not found", &assistant_name))
    }
}
