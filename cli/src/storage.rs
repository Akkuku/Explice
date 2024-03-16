use anyhow::Context;
use lib::{ChatRecordStorage, ExpliceConfigStorage, LocalAssistants, APP_NAME};
use persist::LocalJsonStorage;
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "config.json";
const ASSISTANTS_FILE_NAME: &str = "assistants.json";
const CHAT_RECORDS_FILE_NAME: &str = "chat_records.json";

pub(crate) struct Storage;

impl Storage {
    pub(crate) fn config() -> anyhow::Result<ExpliceConfigStorage<LocalJsonStorage>> {
        let storage = LocalJsonStorage::new(user_config_path(CONFIG_FILE_NAME)?);
        let config_storage = ExpliceConfigStorage::new(storage);

        Ok(config_storage)
    }

    pub(crate) fn assistants() -> anyhow::Result<LocalAssistants<LocalJsonStorage>> {
        let storage = LocalJsonStorage::new(user_config_path(ASSISTANTS_FILE_NAME)?);
        let local_assistants = LocalAssistants::new(storage);

        Ok(local_assistants)
    }

    pub(crate) fn chat_records() -> anyhow::Result<ChatRecordStorage<LocalJsonStorage>> {
        let storage = LocalJsonStorage::new(user_config_path(CHAT_RECORDS_FILE_NAME)?);
        let chat_records = ChatRecordStorage::new(storage);

        Ok(chat_records)
    }
}

fn user_config_path<P: AsRef<Path>>(file_name: P) -> anyhow::Result<PathBuf> {
    let path = dirs::config_dir()
        .context("could not find config directory for your system")?
        .join(APP_NAME)
        .join(file_name);

    Ok(path)
}
