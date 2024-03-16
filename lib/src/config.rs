use crate::storage::Storage;
use crate::APP_NAME;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpliceConfig {
    api_key: String,
    token_limit: u16,
}

impl ExpliceConfig {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn token_limit(&self) -> &u16 {
        &self.token_limit
    }

    pub fn new(api_key: String, token_limit: u16) -> Self {
        ExpliceConfig {
            api_key,
            token_limit,
        }
    }

    fn update(&mut self, api_key: Option<&str>, token_limit: Option<u16>) {
        if let Some(api_key) = api_key {
            self.api_key = api_key.to_owned();
        };
        if let Some(token_limit) = token_limit {
            self.token_limit = token_limit.to_owned();
        };
    }
}

pub struct ExpliceConfigStorage<S>
where
    S: Storage<ExpliceConfig>,
{
    storage: S,
}

impl<S> ExpliceConfigStorage<S>
where
    S: Storage<ExpliceConfig>,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn is_initialized(&self) -> bool {
        self.read().is_ok()
    }

    pub fn init(&self, api_key: String, token_limit: u16) -> Result<()> {
        let config = ExpliceConfig::new(api_key, token_limit);
        self.storage.write(&config)
    }

    pub fn update(&self, api_key: Option<&str>, token_limit: Option<u16>) -> Result<()> {
        let mut config = self.storage.read()?.context("no config found")?;
        config.update(api_key, token_limit);
        self.storage.write(&config)
    }

    pub fn read(&self) -> Result<ExpliceConfig> {
        self.storage
            .read()?
            .with_context(|| format!("no config found, run \"{} config init\" first", APP_NAME))
    }
}
