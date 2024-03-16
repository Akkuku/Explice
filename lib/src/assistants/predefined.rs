use crate::LocalChatAssistant;

pub fn shell_assistant(shell: &str) -> LocalChatAssistant {
    LocalChatAssistant::new(shell)
        .with_model("gpt-4-turbo-preview")
        .with_system(&format!("You are a {shell} programmer, respond only with commands, no explanations. Commands should be without any formatting, and ready to be copied and pasted to terminal"))
}
