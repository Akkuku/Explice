#![feature(absolute_path)]

mod ai;
mod assistant;
mod completion;
mod config;
pub(crate) mod persist;
mod placeholder;
pub mod validation;

pub use ai::*;
pub use assistant::*;
pub use completion::*;
pub use config::*;
pub use placeholder::*;

pub const APP_NAME: &str = "explice";
