use std::fs;
use std::path::Path;
use dialoguer::Confirm;
use serde::{Serialize, Deserialize};

const CONFIG_PATH: &str = "Conf.toml";

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    apikey: String
}

impl Config {
    pub fn new(apikey: String) -> Self {
        Config { apikey }
    }
}

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

    let config = Config::new(apikey);
    let toml = toml::to_string(&config).unwrap();
    if let Err(err) = fs::write(config_path, toml){
        eprintln!("{:?}", err);
    }
    println!("Successfully initialized in current directory");
}