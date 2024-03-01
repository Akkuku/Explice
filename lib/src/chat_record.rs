use crate::persist::PersistConfig;
use chrono::{DateTime, Local};
use std::fmt::Display;

pub struct ChatRecord {
    assistant_name: String,
    creation_date: DateTime<Local>,
    messages: Vec<ChatMessage>,
}

impl ChatRecord {
    pub fn new(assistant_name: &str) -> Self {
        Self {
            assistant_name: assistant_name.to_owned(),
            creation_date: Local::now(),
            messages: Default::default(),
        }
    }

    pub(crate) fn with_messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = messages;
        self
    }

    pub fn add_user(&mut self, message: &str) {
        self.messages.push(ChatMessage::new_user(message))
    }

    pub fn add_assistant(&mut self, message: &str) {
        self.messages.push(ChatMessage::new_assistant(message))
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let file_name = format!(
            "{}-{}.txt",
            self.assistant_name,
            self.creation_date.date_naive().to_string()
        );

        let content = self.to_string();
        PersistConfig::save(&file_name, &content)
    }
}

impl Display for ChatRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let messages = self
            .messages
            .iter()
            .map(|msg| msg.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        writeln!(
            f,
            "Conversation with assistant \"{}\" on {}",
            self.assistant_name,
            self.creation_date.to_string(),
        )?;
        writeln!(f, "{}", messages)
    }
}

pub(crate) struct ChatMessage {
    role: Role,
    content: String,
}

enum Role {
    User,
    Assistant,
}

impl ChatMessage {
    pub fn new_assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_owned(),
        }
    }

    pub fn new_user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_owned(),
        }
    }
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self.role {
            Role::User => "User",
            Role::Assistant => "Assistant",
        };
        write!(f, "{}: {}", role, self.content)
    }
}
