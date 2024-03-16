use crate::dialog::{input_chat_prompt, select_assistant};
use crate::storage::Storage;
use anyhow::{Context, Result};
use clap::Args;
use dialoguer::BasicHistory;
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

    let (create_prompt, handle_completion) = prepare_chat_loop();

    let chat_record = open_ai
        .chat()
        .create_loop(&config, &assistant, create_prompt, handle_completion)
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

    let (create_prompt, handle_completion) = prepare_chat_loop();

    let chat_record = open_ai
        .chat()
        .create_loop_with_thread(&assistant, create_prompt, handle_completion)
        .await?;

    Storage::chat_records()?.save(chat_record)?;

    Ok(())
}

fn prepare_chat_loop() -> (impl FnMut() -> Result<Option<String>>, impl Fn(&str) -> ()) {
    println!("Enter your prompt below. Leave it blank to exit");

    let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);
    let create_prompt = move || input_chat_prompt(&mut history);
    let handle_completion = |completion: &str| println!("{completion}");

    (create_prompt, handle_completion)
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
