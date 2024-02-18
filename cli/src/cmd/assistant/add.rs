use crate::dialog::select_model;
use clap::Args;
use lib::{get_available_chat_models, AssistantData, ExpliceConfig};

#[derive(Debug, Args)]
pub struct AddAssistantArgs {
    #[arg(long, short)]
    name: String,
    #[arg(long, short)]
    model: Option<String>,
    #[arg(long, short, default_value = "")]
    description: String,
    #[arg(long, short, default_value = "You are a helpful assistant")]
    system: String,
}

impl From<AddAssistantArgs> for AssistantData {
    fn from(assistant: AddAssistantArgs) -> Self {
        Self {
            name: assistant.name,
            model: assistant.model.unwrap(),
            description: assistant.description,
            system: assistant.system,
        }
    }
}

pub(crate) async fn assistant_add_cmd(mut args: AddAssistantArgs) -> anyhow::Result<()> {
    let mut config = ExpliceConfig::read()?;

    if args.model.is_none() {
        let models = get_available_chat_models(config.apikey()).await?;
        let model = select_model(models)?;
        args.model = Some(model);
    }

    config
        .push_assistant(AssistantData::from(args).into())?
        .save()?;

    println!("Successfully added assistant");
    Ok(())
}
