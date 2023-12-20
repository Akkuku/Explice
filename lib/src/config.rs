use crate::{ChatAssistant, ChatAssistants, Persist, APP_NAME};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpliceConfig {
    apikey: String,
    token_limit: u16,
    #[serde(flatten)]
    assistants: ChatAssistants,
}

impl ExpliceConfig {
    fn path() -> Result<PathBuf> {
        let file_path = dirs::config_dir()
            .context("could not find config directory for your system")?
            .join(APP_NAME)
            .join(CONFIG_FILE_NAME);

        Ok(file_path)
    }

    pub fn exists() -> Result<bool> {
        let exists = Self::path()?.try_exists()?;
        Ok(exists)
    }

    pub fn new(apikey: &str, token_limit: &u16) -> Self {
        ExpliceConfig {
            apikey: apikey.to_owned(),
            token_limit: token_limit.to_owned(),
            assistants: Default::default(),
        }
    }

    pub fn update(&mut self, apikey: Option<&str>, token_limit: Option<u16>) -> &mut Self {
        if let Some(apikey) = apikey {
            self.apikey = apikey.to_owned();
        };
        if let Some(token_limit) = token_limit {
            self.token_limit = token_limit.to_owned();
        };

        self
    }

    pub fn apikey(&self) -> &str {
        &self.apikey
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
}

impl Persist for ExpliceConfig {
    fn read() -> Result<Self> {
        let path = Self::path()?;
        if !path.try_exists()? {
            bail!(
                "Config not found, run {} init first",
                APP_NAME.to_lowercase()
            )
        }

        let content = fs::read_to_string(path)?;
        let config = serde_json::from_str(content.as_str())?;

        Ok(config)
    }

    fn save(&self) -> Result<()> {
        let path = Self::path()?;
        let dir = path.parent().unwrap();
        if !dir.try_exists()? {
            fs::create_dir_all(dir)?;
        }

        let json = serde_json::to_string_pretty(&self)?;
        fs::write(path, json)?;

        Ok(())
    }
}
