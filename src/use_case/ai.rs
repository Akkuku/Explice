use crate::persistence::assistants::ChatAssistant;
use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;

pub(crate) async fn create_chat_completion(
    apikey: String,
    prompt: String,
    assistant: ChatAssistant,
) -> Result<String> {
    let open_ai_config = OpenAIConfig::new().with_api_key(apikey);
    let client = Client::with_config(open_ai_config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(assistant.model())
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(assistant.system())
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ])
        .max_tokens(40u16)
        .build()?;

    let response = client.chat().create(request).await?;
    Ok(response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .as_ref()
        .unwrap()
        .to_string())
}
