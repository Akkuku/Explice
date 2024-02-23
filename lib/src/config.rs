use crate::persist::PersistConfig;
use crate::{ChatAssistant, ChatAssistants};
use anyhow::Result;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpliceConfig {
    api_key: String,
    token_limit: u16,
    #[serde(flatten)]
    assistants: ChatAssistants,
}

impl ExpliceConfig {
    pub fn new(api_key: String, token_limit: u16) -> Self {
        ExpliceConfig {
            api_key,
            token_limit,
            assistants: Default::default(),
        }
    }

    pub fn update(&mut self, api_key: Option<&str>, token_limit: Option<u16>) -> &mut Self {
        if let Some(api_key) = api_key {
            self.api_key = api_key.to_owned();
        };
        if let Some(token_limit) = token_limit {
            self.token_limit = token_limit.to_owned();
        };

        self
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn token_limit(&self) -> &u16 {
        &self.token_limit
    }

    pub fn push_assistant(&mut self, assistant: ChatAssistant) -> Result<&Self> {
        self.assistants.push(assistant)?;
        Ok(self)
    }

    pub fn assistants(&self) -> &ChatAssistants {
        &self.assistants
    }

    pub fn read() -> Result<Self> {
        PersistConfig::read(CONFIG_FILE_NAME)
    }

    pub fn save(&self) -> Result<()> {
        PersistConfig::save(CONFIG_FILE_NAME, &self)
    }

    pub fn exists() -> Result<bool> {
        PersistConfig::exists(CONFIG_FILE_NAME)
    }
}
