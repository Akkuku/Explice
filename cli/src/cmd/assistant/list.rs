use lib::{ExpliceConfig, Persist};

pub fn assistant_list_cmd() -> anyhow::Result<()> {
    let config = ExpliceConfig::read()?;
    println!("Available assistants: {:?}", config.assistants().names());
    Ok(())
}