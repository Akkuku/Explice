use crate::cmd::assistant::Assistant;
use crate::dialog::select_model;
use lib::{get_available_chat_models, AssistantData, ExpliceConfig, Persist};

pub(crate) async fn assistant_add_cmd(mut cli_assistant: Assistant) -> anyhow::Result<()> {
    let mut config = ExpliceConfig::read()?;

    if cli_assistant.model.is_none() {
        let models = get_available_chat_models(config.apikey()).await?;
        let model = select_model(models)?;
        cli_assistant.model = Some(model);
    }

    config
        .push_assistant(AssistantData::from(cli_assistant).into())?
        .save()?;

    println!("Successfully added assistant");
    Ok(())
}
