use crate::persistence::config::ExpliceConfig;
use crate::use_case::ai::create_chat_completion;

pub(crate) async fn chat_cmd(prompt: String) -> anyhow::Result<()> {
    let config = ExpliceConfig::read()?;
    let completion = create_chat_completion(config.apikey().to_string(), prompt).await?;

    println!("{completion}");
    Ok(())
}
