use crate::chat_controller::ChatLoopController;
use crate::dialog::select_assistant;
use crate::storage::Storage;
use anyhow::{Context, Result};
use clap::Args;
use lib::{ChatAssistant, OpenAi};

#[derive(Debug, Args)]
pub struct ChatArgs {
    #[arg(long = "assistant", short)]
    assistant_name: Option<String>,
    #[arg(long, short)]
    thread: bool,
}

pub(crate) async fn chat_cmd(args: ChatArgs) -> Result<()> {
    match args.thread {
        true => chat_thread(args).await,
        false => chat(args).await,
    }
}

pub(crate) async fn chat(args: ChatArgs) -> Result<()> {
    let config = Storage::config()?.read()?;
    let open_ai = OpenAi::new(&config.api_key());

    let mut assistants = Storage::assistants()?.list()?;
    let mut open_ai_assistants = open_ai.assistants().list().await?;
    assistants.append(&mut open_ai_assistants);

    let assistant = get_or_select_assistant(args.assistant_name, assistants)?;

    let chat_record = open_ai
        .chat(ChatLoopController::default())
        .create_loop(&config, &assistant)
        .await?;

    Storage::chat_records()?.save(chat_record)?;

    Ok(())
}

async fn chat_thread(args: ChatArgs) -> Result<()> {
    let config = Storage::config()?.read()?;
    let open_ai = OpenAi::new(&config.api_key());

    let assistants = open_ai.assistants().list().await?;
    let assistant = get_or_select_assistant(args.assistant_name, assistants)?
        .external()
        .context("only external assistants can use threads")?;

    let chat_record = open_ai
        .chat(ChatLoopController::default())
        .create_loop_with_thread(&assistant)
        .await?;

    Storage::chat_records()?.save(chat_record)?;

    Ok(())
}

fn get_or_select_assistant(
    assistant_name: Option<String>,
    assistants: Vec<ChatAssistant>,
) -> Result<ChatAssistant> {
    let assistant = match assistant_name {
        None => select_assistant(assistants)?,
        Some(assistant_name) => assistants
            .into_iter()
            .find(|a| a.name() == &assistant_name)
            .context("assistant not found")?,
    };

    Ok(assistant)
}
