use lib::{ExpliceConfig, OpenAi};

pub(crate) async fn assistant_list_cmd() -> anyhow::Result<()> {
    let config = ExpliceConfig::read()?;
    let open_ai = OpenAi::new(&config.api_key());
    let assistants = open_ai.assistant_names().await?;
    println!("Local assistants: {:?}", config.assistants().names());
    println!("OpenAi assistants: {:?}", assistants);
    Ok(())
}
