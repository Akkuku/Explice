use crate::APP_NAME;
use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub(crate) struct PersistConfig;

impl PersistConfig {
    pub fn read_json<T>(file_name: &str) -> anyhow::Result<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let content = Self::read(file_name)?;
        serde_json::from_str(&content).map_err(anyhow::Error::from)
    }

    pub fn save_json<T>(file_name: &str, content: &T) -> anyhow::Result<()>
    where
        T: Sized + Serialize,
    {
        let json = serde_json::to_string(&content)?;
        Self::save(file_name, &json)
    }

    pub fn read(file_name: &str) -> anyhow::Result<String> {
        let path = Self::path(file_name)?;
        if !path.try_exists()? {
            bail!("Config not found, run \"{APP_NAME} config\" first")
        }

        let content = fs::read_to_string(path)?;

        Ok(content)
    }

    pub fn save(file_name: &str, content: &str) -> anyhow::Result<()> {
        let path = Self::path(file_name)?;
        let dir = path.parent().unwrap();
        if !dir.try_exists()? {
            fs::create_dir_all(dir)?;
        }

        fs::write(path, content)?;

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
