use crate::dialog::confirm_override;
use lib::{ExpliceConfig, Persist};

pub fn init_cmd(apikey: &str) -> anyhow::Result<()> {
    if ExpliceConfig::exists()? {
        println!("Config already exists");

        if !confirm_override()? {
            return Ok(());
        }
    }

    let config = ExpliceConfig::new(apikey);
    config.save()?;

    println!("Successfully initialized");
    Ok(())
}
