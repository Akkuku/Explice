use anyhow::Result;
use lib::{create_chat_completion, ExpliceConfig, Persist};

pub async fn chat_cmd(prompt: Vec<String>, assistant_name: &str) -> Result<()> {
    let config = ExpliceConfig::read()?;
    let apikey = config.apikey();
    let assistant = config.assistants().get_by_name(assistant_name)?;

    let completion = create_chat_completion(apikey, &prompt.join(" "), assistant).await?;
    println!("{completion}");

    Ok(())
}
