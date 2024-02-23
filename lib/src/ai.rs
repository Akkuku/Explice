use crate::assistant::ChatAssistant;
use crate::{replace_placeholders, ExpliceConfig};
use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;

pub async fn create_chat_completion(
    api_key: &str,
    token_limit: &u16,
    assistant: &ChatAssistant,
    messages: Vec<ChatCompletionRequestMessage>,
) -> Result<String> {
    let open_ai_config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(open_ai_config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(assistant.model())
        .messages(messages)
        .max_tokens(*token_limit)
        .build()?;

    let response = client.chat().create(request).await?;
    let completion = response
        .choices
        .first()
        .expect("model returned no choices")
        .message
        .content
        .as_ref()
        .expect("message content is empty")
        .to_string();

    Ok(completion)
}

pub async fn get_available_chat_models(api_key: &str) -> Result<Vec<String>> {
    let chat_models = get_available_models(api_key)
        .await?
        .into_iter()
        .filter(|name| name.starts_with("gpt"))
        .collect();

    Ok(chat_models)
}

async fn get_available_models(api_key: &str) -> Result<Vec<String>> {
    let open_ai_config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(open_ai_config);

    let models = client
        .models()
        .list()
        .await?
        .data
        .iter()
        .map(|model| model.id.to_owned())
        .collect();

    Ok(models)
}

pub async fn create_chat_loop<F>(
    config: &ExpliceConfig,
    assistant: &ChatAssistant,
    create_prompt: F,
) -> Result<()>
where
    F: Fn() -> Result<Option<String>>,
{
    println!("Enter your prompt below. Leave it blank to exit");

    let mut message_builder = ChatMessageBuilder::new(assistant.system())?;
    loop {
        let prompt = match create_prompt()? {
            None => break,
            Some(prompt) => replace_placeholders(prompt)?,
        };

        message_builder.add_user(&prompt)?;
        let completion = create_chat_completion(
            config.api_key(),
            config.token_limit(),
            assistant,
            message_builder.build().to_vec(),
        )
        .await?;
        message_builder.add_assistant(&completion)?;

        println!("{completion}");
    }

    Ok(())
}

struct ChatMessageBuilder {
    messages: Vec<ChatCompletionRequestMessage>,
}

impl ChatMessageBuilder {
    pub fn new(system_message: &str) -> Result<Self> {
        Ok(Self {
            messages: vec![ChatCompletionRequestSystemMessageArgs::default()
                .content(system_message)
                .build()?
                .into()],
        })
    }

    pub fn add_user(&mut self, prompt: &str) -> Result<&mut Self> {
        self.messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );
        Ok(self)
    }

    pub fn add_assistant(&mut self, completion: &str) -> Result<&mut Self> {
        self.messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(completion)
                .build()?
                .into(),
        );
        Ok(self)
    }

    pub fn build(&self) -> &Vec<ChatCompletionRequestMessage> {
        &self.messages
    }
}
