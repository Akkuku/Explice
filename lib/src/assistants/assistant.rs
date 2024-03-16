use crate::{LocalChatAssistant, OpenAiChatAssistant};

pub enum ChatAssistant {
    LocalAssistant(LocalChatAssistant),
    ExternalAssistant(OpenAiChatAssistant),
}

impl ChatAssistant {
    pub fn name(&self) -> &str {
        match self {
            ChatAssistant::LocalAssistant(local_assistant) => local_assistant.name(),
            ChatAssistant::ExternalAssistant(external_assistant) => external_assistant.name(),
        }
    }

    pub fn model(&self) -> &str {
        match self {
            ChatAssistant::LocalAssistant(local_assistant) => local_assistant.model(),
            ChatAssistant::ExternalAssistant(external_assistant) => external_assistant.model(),
        }
    }

    pub fn system(&self) -> &str {
        match self {
            ChatAssistant::LocalAssistant(local_assistant) => local_assistant.system(),
            ChatAssistant::ExternalAssistant(external_assistant) => external_assistant.system(),
        }
    }

    pub fn external(self) -> Option<OpenAiChatAssistant> {
        match self {
            ChatAssistant::LocalAssistant(_) => None,
            ChatAssistant::ExternalAssistant(assistant) => Some(assistant),
        }
    }
}
