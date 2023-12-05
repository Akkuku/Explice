use crate::persistence::config::ExpliceConfig;
use crate::use_case::ai::create_chat_completion;

pub(crate) async fn chat_cmd(prompt: String) {
    let config = match ExpliceConfig::read() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };

    match create_chat_completion(config.apikey().to_string(), prompt).await {
        Ok(completion) => {
            println!("{completion}");
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };
}
