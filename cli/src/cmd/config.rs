use crate::dialog::input_api_key;
use crate::storage::Storage;
use anyhow::bail;
use clap::Args;
use lib::validation::{openai_api_key_format_validator, openai_api_key_request_validator};
use lib::ExpliceConfigStorage;
use persist::LocalJsonStorage;

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[arg(long, short)]
    api_key: Option<String>,
    #[arg(long, short)]
    token_limit: Option<u16>,
}

impl ConfigArgs {
    pub(crate) fn are_empty(&self) -> bool {
        self.api_key.is_none() && self.token_limit.is_none()
    }
}

pub(crate) async fn config_cmd(args: ConfigArgs) -> anyhow::Result<()> {
    let config_storage = Storage::config()?;
    match config_storage.is_initialized() {
        true => update_config(args, config_storage).await?,
        false => create_config(args, config_storage).await?,
    };

    Ok(())
}

async fn update_config(
    args: ConfigArgs,
    config_storage: ExpliceConfigStorage<LocalJsonStorage>,
) -> anyhow::Result<()> {
    if args.are_empty() {
        bail!("Config exists, provide some arguments for update");
    }

    if let Some(api_key) = &args.api_key {
        openai_api_key_format_validator(&api_key)?;
        openai_api_key_request_validator(&api_key).await?;
    };

    config_storage.update(args.api_key.as_deref(), args.token_limit)?;

    println!("Successfully updated config");
    Ok(())
}

async fn create_config(
    args: ConfigArgs,
    config_storage: ExpliceConfigStorage<LocalJsonStorage>,
) -> anyhow::Result<()> {
    let api_key = match args.api_key {
        None => input_api_key()?,
        Some(api_key) => api_key,
    };

    openai_api_key_format_validator(&api_key)?;
    openai_api_key_request_validator(&api_key).await?;

    let token_limit = args.token_limit.unwrap_or(40);

    config_storage.init(api_key, token_limit)?;
    Storage::assistants()?.init()?;

    println!("Successfully initialized");
    Ok(())
}
