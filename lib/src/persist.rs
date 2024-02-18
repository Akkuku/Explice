use crate::APP_NAME;
use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct PersistConfig;

impl PersistConfig {
    pub fn read<T: for<'a> Deserialize<'a>>(file_name: &str) -> anyhow::Result<T> {
        let path = Self::path(file_name)?;
        if !path.try_exists()? {
            bail!("Config not found, run {APP_NAME} init first")
        }

        let content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;

        Ok(config)
    }

    pub fn save<T: Sized + Serialize>(file_name: &str, content: &T) -> anyhow::Result<()> {
        let path = Self::path(file_name)?;
        let dir = path.parent().unwrap();
        if !dir.try_exists()? {
            fs::create_dir_all(dir)?;
        }

        let json = serde_json::to_string(&content)?;
        fs::write(path, json)?;

        Ok(())
    }

    pub fn exists(file_name: &str) -> anyhow::Result<bool> {
        let exists = Self::path(file_name)?.try_exists()?;
        Ok(exists)
    }

    fn path(file_name: &str) -> anyhow::Result<PathBuf> {
        let dir_path = dirs::config_dir()
            .context("could not find config directory for your system")?
            .join(APP_NAME)
            .join(file_name);

        Ok(dir_path)
    }
}
