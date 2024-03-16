use anyhow::bail;
use lib::{KVStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct LocalJsonStorage {
    path: PathBuf,
}

impl LocalJsonStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl<T> Storage<T> for LocalJsonStorage
where
    T: Serialize + for<'d> Deserialize<'d>,
{
    fn write(&self, item: &T) -> anyhow::Result<()> {
        let dir = &self.path.parent().unwrap();
        if !dir.try_exists()? {
            fs::create_dir_all(dir)?;
        }

        let json = serde_json::to_string(&item)?;
        fs::write(&self.path, json).map_err(anyhow::Error::from)
    }

    fn read(&self) -> anyhow::Result<Option<T>> {
        if !&self.path.try_exists()? {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.path)?;
        let data = serde_json::from_str(&content)?;

        Ok(Some(data))
    }
}

impl<V> KVStorage<String, V> for LocalJsonStorage
where
    V: Serialize + for<'d> Deserialize<'d>,
{
    fn add(&self, key: String, value: V) -> anyhow::Result<()> {
        let mut content: HashMap<String, V> = self.read()?.unwrap_or_default();

        if content.contains_key(&key) {
            bail!("key \"{key}\" already exists");
        }

        content.insert(key, value);

        self.write(&content)
    }

    fn get(&self, key: String) -> anyhow::Result<Option<V>> {
        let Some(mut content): Option<HashMap<String, V>> = self.read()? else {
            return Ok(None);
        };

        Ok(content.remove(&key))
    }

    fn get_all(&self) -> anyhow::Result<Vec<V>> {
        let Some(content): Option<HashMap<String, V>> = self.read()? else {
            return Ok(vec![]);
        };

        let values = content.into_values().collect();

        Ok(values)
    }

    fn update(&self, key: String, value: V) -> anyhow::Result<()> {
        let mut content: HashMap<String, V> = self.read()?.unwrap_or_default();

        if !content.contains_key(&key) {
            bail!("key \"{key}\" does not exist");
        }

        content.insert(key, value);

        self.write(&content)
    }

    fn delete(&self, key: String) -> anyhow::Result<()> {
        let mut content: HashMap<String, V> = self.read()?.unwrap_or_default();

        if !content.contains_key(&key) {
            bail!("key \"{key}\" does not exist");
        }

        content.remove(&key);

        self.write(&content)
    }
}
