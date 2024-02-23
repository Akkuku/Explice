use crate::dialog::select_model;
use clap::Args;
use lib::{AssistantData, ExpliceConfig, OpenAi};

#[derive(Debug, Args)]
pub struct AssistantAddArgs {
    #[arg(long, short)]
    name: String,
    #[arg(long, short)]
    model: Option<String>,
    #[arg(long, short, default_value = "")]
    description: String,
    #[arg(long, short, default_value = "You are a helpful assistant")]
    system: String,
}

impl From<AssistantAddArgs> for AssistantData {
    fn from(assistant: AssistantAddArgs) -> Self {
        Self {
            name: assistant.name,
            model: assistant.model.unwrap(),
            description: assistant.description,
            system: assistant.system,
        }
    }
}

pub(crate) async fn assistant_add_cmd(mut args: AssistantAddArgs) -> anyhow::Result<()> {
    let mut config = ExpliceConfig::read()?;

    if args.model.is_none() {
        let models = OpenAi::new(&config.api_key()).chat_models().await?;
        let model = select_model(models)?;
        args.model = Some(model);
    }

    config
        .push_assistant(AssistantData::from(args).into())?
        .save()?;

    println!("Successfully added assistant");
    Ok(())
}
