use anyhow::bail;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequestArgs, MessageContent,
    RunObject, RunStatus,
};
use async_openai::Client;
use std::time::Duration;

pub struct Thread {
    id: String,
    client: Client<OpenAIConfig>,
}

impl Thread {
    pub async fn new(open_ai_client: &Client<OpenAIConfig>) -> anyhow::Result<Self> {
        let request = CreateThreadRequestArgs::default().build()?;
        let thread = open_ai_client.threads().create(request).await?;

        Ok(Self {
            id: thread.id,
            client: open_ai_client.to_owned(),
        })
    }

    pub async fn retrieve_run(&self, run_id: &str) -> anyhow::Result<RunObject> {
        let run = self
            .client
            .threads()
            .runs(&self.id)
            .retrieve(run_id)
            .await?;

        Ok(run)
    }

    pub async fn chat_completion(
        &self,
        prompt: &str,
        assistant_id: &str,
    ) -> anyhow::Result<String> {
        let user_message_id = self.add_user_message(prompt).await?;
        self.run_till_completion(assistant_id).await?;
        let completion = self.receive_assistant_response(&user_message_id).await?;

        Ok(completion)
    }

    pub async fn add_user_message(&self, prompt: &str) -> anyhow::Result<String> {
        let message_request = CreateMessageRequestArgs::default()
            .content(prompt)
            .build()?;
        let message = self
            .client
            .threads()
            .messages(&self.id)
            .create(message_request)
            .await?;

        Ok(message.id)
    }

    pub async fn run_till_completion(&self, assistant_id: &str) -> anyhow::Result<()> {
        let run_id = self.run(assistant_id).await?;
        self.await_run_completion(&run_id).await
    }

    pub async fn receive_assistant_response(
        &self,
        user_message_id: &str,
    ) -> anyhow::Result<String> {
        let query = [("limit", "1"), ("before", user_message_id)];
        let message_response = self
            .client
            .threads()
            .messages(&self.id)
            .list(&query)
            .await?;

        let text = match &message_response.data[0].content[0] {
            MessageContent::Text(text) => text.text.value.clone(),
            MessageContent::ImageFile(_) => bail!("imaged are not supported in the terminal"),
        };

        Ok(text)
    }

    async fn run(&self, assistant_id: &str) -> anyhow::Result<String> {
        let request = CreateRunRequestArgs::default()
            .assistant_id(assistant_id)
            .build()?;
        let run = self.client.threads().runs(&self.id).create(request).await?;

        Ok(run.id)
    }

    pub async fn await_run_completion(&self, run_id: &str) -> anyhow::Result<()> {
        loop {
            let run = self.retrieve_run(run_id).await?;
            match run.status {
                RunStatus::RequiresAction => {
                    bail!("action required, detail: {:?}", run.required_action)
                }
                RunStatus::Cancelling => bail!("assistant response was cancelled"),
                RunStatus::Cancelled => bail!("assistant response was cancelled"),
                RunStatus::Failed => bail!(
                    "failed to receive assistant response, detail: {:?}",
                    run.last_error
                ),
                RunStatus::Expired => bail!("assistant took too long to response"),
                RunStatus::Completed => {
                    break;
                }
                _ => {}
            };
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }
}
