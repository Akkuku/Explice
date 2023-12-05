use crate::persistence::config::{ExpliceConfig, CONFIG_PATH};
use dialoguer::Confirm;
use std::path::Path;

pub(crate) fn init_cmd(apikey: String) {
    let config_path = Path::new(CONFIG_PATH);
    if Path::exists(&config_path) {
        println!("Config already exists");
        let confirmation = Confirm::new()
            .with_prompt("Do you want to override?")
            .default(false)
            .interact()
            .unwrap();

        if !confirmation {
            return;
        }
    }

    let config = ExpliceConfig::new(apikey);
    if let Err(err) = config.save() {
        eprintln!("{:?}", err);
    }
    println!("Successfully initialized in current directory");
}
