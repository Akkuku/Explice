use crate::dialog::{confirm_execute, input_chat_prompt};
use anyhow::Context;
use dialoguer::BasicHistory;
use lib::ChatController;
use std::process::Command;

pub(crate) struct ChatLoopController {
    history: BasicHistory,
}

impl ChatLoopController {
    pub(crate) fn default() -> Self {
        println!("Enter your prompt below. Leave it blank to cancel");
        Self {
            history: BasicHistory::new().max_entries(8).no_duplicates(true),
        }
    }
}

impl ChatController for ChatLoopController {
    fn create_prompt(&mut self) -> anyhow::Result<Option<String>> {
        input_chat_prompt(&mut self.history)
    }

    fn on_completion(&self, completion: &str) -> anyhow::Result<()> {
        println!("{completion}");
        Ok(())
    }
}

pub(crate) struct ExecuteLoopController<'s> {
    history: BasicHistory,
    skip_confirmation: bool,
    shell: &'s str,
}

impl<'s> ExecuteLoopController<'s> {
    pub(crate) fn new(shell: &'s str, skip_confirmation: bool) -> Self {
        println!("Enter your prompt below. Leave it blank to cancel");
        Self {
            history: BasicHistory::new().max_entries(8).no_duplicates(true),
            skip_confirmation,
            shell,
        }
    }
}

impl<'s> ChatController for ExecuteLoopController<'s> {
    fn create_prompt(&mut self) -> anyhow::Result<Option<String>> {
        input_chat_prompt(&mut self.history)
    }

    fn on_completion(&self, completion: &str) -> anyhow::Result<()> {
        println!("{completion}");

        if self.skip_confirmation || confirm_execute()? {
            return execute(self.shell, &completion);
        }

        Ok(())
    }
}

fn execute(shell: &str, completion: &str) -> anyhow::Result<()> {
    let output = Command::new(shell)
        .args(&["-Command", &completion])
        .spawn()
        .context("Failed to execute command")?
        .wait_with_output()?;

    if !output.stdout.is_empty() {
        println!("{:?}", output.stdout);
    }

    if !output.stderr.is_empty() {
        eprintln!("{:?}", output.stderr);
    }

    Ok(())
}
