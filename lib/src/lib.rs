#![feature(absolute_path)]

mod ai;
mod assistant;
mod completion;
mod config;
mod persist;
mod placeholder;

pub use ai::*;
pub use assistant::*;
pub use completion::*;
pub use config::*;
pub use persist::*;
pub use placeholder::*;

pub const APP_NAME: &str = "explice";
