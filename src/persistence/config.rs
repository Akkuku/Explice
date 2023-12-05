use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

pub(crate) const CONFIG_PATH: &str = "Conf.toml";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ExpliceConfig {
    apikey: String,
}

impl ExpliceConfig {
    pub(crate) fn new(apikey: String) -> Self {
        ExpliceConfig { apikey }
    }

    pub(crate) fn read() -> Result<Self> {
        let content = fs::read_to_string(CONFIG_PATH)?;
        let config: ExpliceConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub(crate) fn save(&self) -> Result<()> {
        let toml = toml::to_string(&self).unwrap();
        fs::write(CONFIG_PATH, toml)?;
        Ok(())
    }

    pub(crate) fn apikey(&self) -> &String {
        &self.apikey
    }
}
