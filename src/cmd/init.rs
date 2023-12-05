use crate::persistence::config::{ExpliceConfig, CONFIG_PATH};
use dialoguer::Confirm;
use std::path::Path;

pub(crate) fn init_cmd(apikey: String) -> anyhow::Result<()> {
    if Path::exists(Path::new(CONFIG_PATH)) {
        println!("Config already exists");
        let confirmation = Confirm::new()
            .with_prompt("Do you want to override?")
            .default(false)
            .interact()?;

        if !confirmation {
            return Ok(());
        }
    }

    let config = ExpliceConfig::new(apikey);
    config.save()?;

    println!("Successfully initialized in current directory");
    Ok(())
}
