use crate::ai::thread::Thread;
use crate::chat_record::ChatMessage;
use crate::{
    replace_placeholders, ChatRecord, ExpliceConfig, ExternalChatAssistant, LocalChatAssistant,
};
use anyhow::Context;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};
use async_openai::Client;

pub struct Chat {
    client: Client<OpenAIConfig>,
}

impl Chat {
    pub fn new(open_ai_client: &Client<OpenAIConfig>) -> Self {
        Self {
            client: open_ai_client.to_owned(),
        }
    }

    pub async fn create_loop<FP, FC>(
        &self,
        config: &ExpliceConfig,
        assistant: &LocalChatAssistant,
        mut create_prompt: FP,
        on_completion: FC,
    ) -> anyhow::Result<ChatRecord>
    where
        FP: FnMut() -> anyhow::Result<Option<String>>,
        FC: Fn(&str) -> (),
    {
        let mut message_builder = ChatMessagesBuilder::new(assistant.system())?;
        loop {
            let prompt = match create_prompt()? {
                None => break,
                Some(prompt) => replace_placeholders(prompt)?,
            };
            message_builder.add_user(&prompt)?;

            let completion = self
                .chat_completion(config.token_limit(), assistant, message_builder.build())
                .await?;
            message_builder.add_assistant(&completion)?;

            on_completion(&completion);
        }

        Ok(message_builder.to_chat_record(assistant.name()))
    }

    pub async fn create_loop_with_thread<FP, FC>(
        &self,
        assistant: &ExternalChatAssistant,
        mut create_prompt: FP,
        on_completion: FC,
    ) -> anyhow::Result<ChatRecord>
    where
        FP: FnMut() -> anyhow::Result<Option<String>>,
        FC: Fn(&str) -> (),
    {
        let mut chat_record = ChatRecord::new(assistant.name());
        let thread = Thread::new(&self.client).await?;

        loop {
            let prompt = match create_prompt()? {
                None => break,
                Some(prompt) => replace_placeholders(prompt)?,
            };
            chat_record.add_user(&prompt);

            let completion = thread.chat_completion(&prompt, &assistant.id()).await?;
            chat_record.add_assistant(&completion);

            on_completion(&completion);
        }

        Ok(chat_record)
    }

    async fn chat_completion(
        &self,
        token_limit: &u16,
        assistant: &LocalChatAssistant,
        messages: Vec<ChatCompletionRequestMessage>,
    ) -> anyhow::Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(assistant.model())
            .messages(messages)
            .max_tokens(*token_limit)
            .build()?;

        let response = self.client.chat().create(request).await?;
        let completion = response
            .choices
            .first()
            .context("model returned no choices")?
            .message
            .content
            .as_ref()
            .context("message content is empty")?
            .to_string();

        Ok(completion)
    }
}

struct ChatMessagesBuilder {
    messages: Vec<ChatCompletionRequestMessage>,
}

impl ChatMessagesBuilder {
    pub(crate) fn new(system_message: &str) -> anyhow::Result<Self> {
        Ok(Self {
            messages: vec![ChatCompletionRequestSystemMessageArgs::default()
                .content(system_message)
                .build()?
                .into()],
        })
    }

    pub(crate) fn add_user(&mut self, prompt: &str) -> anyhow::Result<&mut Self> {
        self.messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );
        Ok(self)
    }

    pub(crate) fn add_assistant(&mut self, completion: &str) -> anyhow::Result<&mut Self> {
        self.messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(completion)
                .build()?
                .into(),
        );
        Ok(self)
    }

    pub(crate) fn build(&self) -> Vec<ChatCompletionRequestMessage> {
        self.messages.to_vec()
    }

    pub(crate) fn to_chat_record(self, assistant_name: &str) -> ChatRecord {
        let messages = self.messages
            .into_iter()
            .filter_map(|message| match message {
                ChatCompletionRequestMessage::User(message) => {
                    let ChatCompletionRequestUserMessageContent::Text(text) = message.content else  {
                        return None;
                    };
                    Some(ChatMessage::new_user(&text))
                }
                ChatCompletionRequestMessage::Assistant(message) => Some(ChatMessage::new_assistant(&message.content?)),
                _ => None
            }).collect();

        ChatRecord::new(assistant_name).with_messages(messages)
    }
}
