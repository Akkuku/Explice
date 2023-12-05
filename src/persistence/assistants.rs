use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub(crate) const ASSISTANTS_PATH: &str = "assistants.json";

#[derive(Serialize, Deserialize)]
pub(crate) struct ChatAssistants {
    assistants: Vec<ChatAssistant>,
}

impl ChatAssistant {
    pub fn model(&self) -> &String {
        &self.model
    }
    pub fn system(&self) -> &String {
        &self.system
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, clap::Args)]
pub(crate) struct ChatAssistant {
    #[arg(long, short)]
    name: String,
    #[arg(long, short)]
    #[arg(default_value = "gpt-3.5-turbo")]
    model: String,
    #[arg(long, short)]
    #[arg(default_value = "")]
    description: String,
    #[arg(long, short)]
    #[arg(default_value = "You are a helpful assistant")]
    system: String,
}

impl Default for ChatAssistants {
    fn default() -> Self {
        ChatAssistants { assistants: vec![] }
    }
}

impl ChatAssistants {
    pub(crate) fn read() -> Result<Self> {
        if !Path::exists(Path::new(ASSISTANTS_PATH)) {
            return Ok(Default::default());
        }
        let content = fs::read_to_string(ASSISTANTS_PATH)?;
        let assistants = serde_json::from_str(&content)?;
        Ok(assistants)
    }

    pub(crate) fn assistant(assistant_name: String) -> Result<Option<ChatAssistant>> {
        let assistants = ChatAssistants::read()?;
        Ok(assistants.get(assistant_name))
    }

    pub(crate) fn save(&self) -> Result<()> {
        let json = serde_json::to_string(&self).unwrap();
        fs::write(ASSISTANTS_PATH, json)?;
        Ok(())
    }

    pub(crate) fn push(&mut self, assistant: ChatAssistant) -> Result<()> {
        if self.assistants.iter().any(|a| a.name == assistant.name) {
            bail!("Assistant with name {:?} already exists", assistant.name);
        }
        self.assistants.push(assistant);
        Ok(())
    }

    pub(crate) fn names(&self) -> Vec<String> {
        self.assistants
            .iter()
            .map(|assistant| assistant.name.to_owned())
            .collect()
    }

    fn get(&self, assistant_name: String) -> Option<ChatAssistant> {
        self.assistants
            .iter()
            .find(|assistant| assistant.name == assistant_name)
            .cloned()
    }
}
