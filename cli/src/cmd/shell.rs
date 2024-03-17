use crate::chat_controller::ExecuteLoopController;
use crate::storage::Storage;
use anyhow::{bail, Result};
use clap::Args;
use lib::predefined::shell_assistant;
use lib::{ChatAssistant, OpenAi};
use std::env;

#[derive(Debug, Args)]
pub struct ShellArgs {
    #[arg(long, short, help = "execute without confirmation")]
    yes: bool,
}

pub(crate) async fn shell_cmd(args: ShellArgs) -> Result<()> {
    let shell = match env::consts::OS {
        "windows" => "powershell",
        "linux" => "bash",
        _ => bail!("your system is not yet supported"),
    };

    let config = Storage::config()?.read()?;
    let assistant = ChatAssistant::LocalAssistant(shell_assistant(shell));

    OpenAi::new(&config.api_key())
        .chat(ExecuteLoopController::new(args.yes))
        .create_loop(&config, &assistant)
        .await?;

    Ok(())
}
