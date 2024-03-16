#![feature(absolute_path)]

mod assistants;
mod chat_record;
mod completion;
mod config;
mod open_ai;
mod placeholder;
pub mod predefined;
mod storage;
pub mod validation;

pub use assistants::*;
pub use chat_record::*;
pub use completion::*;
pub use config::*;
pub use open_ai::*;
pub use placeholder::*;
pub use storage::{KVStorage, Storage};

pub const APP_NAME: &str = "explice";
