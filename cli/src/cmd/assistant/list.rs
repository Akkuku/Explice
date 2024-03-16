use crate::storage::Storage;
use lib::OpenAi;

pub(crate) async fn assistant_list_cmd() -> anyhow::Result<()> {
    let config = Storage::config()?.read()?;
    let open_ai = OpenAi::new(&config.api_key());

    let local_assistant_names = Storage::assistants()?.names()?;
    let external_assistant_names = open_ai.assistants().names().await?;

    println!("Local assistants: {local_assistant_names:?}");
    println!("OpenAi assistants: {external_assistant_names:?}");
    Ok(())
}
