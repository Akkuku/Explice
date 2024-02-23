use crate::APP_NAME;
use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub(crate) struct PersistConfig;

impl PersistConfig {
    pub fn read<T>(file_name: &str) -> anyhow::Result<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let path = Self::path(file_name)?;
        if !path.try_exists()? {
            bail!("Config not found, run \"{APP_NAME} config\" first")
        }

        let content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;

        Ok(config)
    }

    pub fn save<T>(file_name: &str, content: &T) -> anyhow::Result<()>
    where
        T: Sized + Serialize,
    {
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
