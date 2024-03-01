use crate::dialog::{input_chat_prompt, select_external_assistant, select_local_assistant};
use anyhow::{Context, Result};
use clap::Args;
use dialoguer::BasicHistory;
use lib::{ExpliceConfig, OpenAi};

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
    let config = ExpliceConfig::read()?;
    let open_ai = OpenAi::new(&config.api_key());

    let assistant = match args.assistant_name {
        None => select_local_assistant(&config)?,
        Some(assistant_name) => config.assistants().get_by_name(&assistant_name)?,
    };

    let (create_prompt, handle_completion) = prepare_chat_loop();

    let chat_record = open_ai
        .chat()
        .create_loop(&config, assistant, create_prompt, handle_completion)
        .await?;

    chat_record.save()?;

    Ok(())
}

async fn chat_thread(args: ChatArgs) -> Result<()> {
    let config = ExpliceConfig::read()?;
    let open_ai = OpenAi::new(&config.api_key());

    let assistant = match args.assistant_name {
        None => select_external_assistant(&open_ai).await?,
        Some(assistant_name) => open_ai
            .assistant_by_name(&assistant_name)
            .await?
            .context("not found assistant")?
            .into(),
    };

    let (create_prompt, handle_completion) = prepare_chat_loop();

    let chat_record = open_ai
        .chat()
        .create_loop_with_thread(&assistant, create_prompt, handle_completion)
        .await?;

    chat_record.save()?;

    Ok(())
}

fn prepare_chat_loop() -> (impl FnMut() -> Result<Option<String>>, impl Fn(&str) -> ()) {
    println!("Enter your prompt below. Leave it blank to exit");

    let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);
    let create_prompt = move || input_chat_prompt(&mut history);
    let handle_completion = |completion: &str| println!("{completion}");

    (create_prompt, handle_completion)
}
