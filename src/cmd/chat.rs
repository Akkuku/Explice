use crate::persistence::assistants::ChatAssistants;
use crate::persistence::config::ExpliceConfig;
use crate::use_case::ai::create_chat_completion;
use anyhow::bail;

pub(crate) async fn chat_cmd(prompt: String, assistant_name: String) -> anyhow::Result<()> {
    let config = ExpliceConfig::read()?;
    let assistant = match ChatAssistants::assistant(assistant_name.to_owned())? {
        Some(assistant) => assistant,
        None => {
            bail!("No assistant with name {assistant_name}")
        }
    };
    let completion = create_chat_completion(config.apikey().to_string(), prompt, assistant).await?;

    println!("{completion}");
    Ok(())
}
