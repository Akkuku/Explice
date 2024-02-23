use crate::ai::thread::Thread;
use crate::{replace_placeholders, ChatAssistant, ExpliceConfig};
use anyhow::Context;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
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
        assistant: &ChatAssistant,
        mut create_prompt: FP,
        on_completion: FC,
    ) -> anyhow::Result<()>
    where
        FP: FnMut() -> anyhow::Result<Option<String>>,
        FC: FnOnce(String) -> () + Copy,
    {
        let mut message_builder = ChatMessageBuilder::new(assistant.system())?;
        loop {
            let prompt = match create_prompt()? {
                None => break,
                Some(prompt) => replace_placeholders(prompt)?,
            };

            message_builder.add_user(&prompt)?;
            let completion = self
                .chat_completion(
                    config.token_limit(),
                    assistant,
                    message_builder.build().to_vec(),
                )
                .await?;
            message_builder.add_assistant(&completion)?;

            on_completion(completion);
        }

        Ok(())
    }

    pub async fn create_loop_with_thread<FP, FC>(
        &self,
        assistant_id: &str,
        mut create_prompt: FP,
        on_completion: FC,
    ) -> anyhow::Result<()>
    where
        FP: FnMut() -> anyhow::Result<Option<String>>,
        FC: FnOnce(String) -> () + Copy,
    {
        let thread = Thread::new(&self.client).await?;

        loop {
            let prompt = match create_prompt()? {
                None => break,
                Some(prompt) => replace_placeholders(prompt)?,
            };

            let completion = thread.chat_completion(&prompt, &assistant_id).await?;

            on_completion(completion);
        }

        Ok(())
    }

    async fn chat_completion(
        &self,
        token_limit: &u16,
        assistant: &ChatAssistant,
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

struct ChatMessageBuilder {
    messages: Vec<ChatCompletionRequestMessage>,
}

impl ChatMessageBuilder {
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

    pub(crate) fn build(&self) -> &Vec<ChatCompletionRequestMessage> {
        &self.messages
    }
}
