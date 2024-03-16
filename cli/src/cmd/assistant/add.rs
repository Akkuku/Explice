use crate::dialog::select_model;
use crate::storage::Storage;
use anyhow::Context;
use clap::Args;
use lib::{AssistantData, OpenAi};

#[derive(Debug, Args)]
pub struct AssistantAddArgs {
    #[arg(long, short)]
    name: String,
    #[arg(long, short)]
    model: Option<String>,
    #[arg(long, short, default_value = "You are a helpful assistant")]
    system: String,
}

pub(crate) async fn assistant_add_cmd(mut args: AssistantAddArgs) -> anyhow::Result<()> {
    let config = Storage::config()?.read()?;

    if args.model.is_none() {
        let models = OpenAi::new(&config.api_key()).chat_models().await?;
        let model = select_model(models)?;
        args.model = Some(model);
    }

    Storage::assistants()?.add(AssistantData::try_from(args)?.into())?;

    println!("Successfully added assistant");
    Ok(())
}

impl TryFrom<AssistantAddArgs> for AssistantData {
    type Error = anyhow::Error;

    fn try_from(assistant: AssistantAddArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            name: assistant.name,
            model: assistant.model.context("assistant model cannot be empty")?,
            system: assistant.system,
        })
    }
}
