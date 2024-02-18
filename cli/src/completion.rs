use dialoguer::Completion;
use lib::get_path_completion;

pub struct PathCompletion;

impl Default for PathCompletion {
    fn default() -> Self {
        Self
    }
}

impl Completion for PathCompletion {
    fn get(&self, input: &str) -> Option<String> {
        match get_path_completion(input) {
            Ok(completion) => Some(completion),
            Err(err) => {
                eprintln!("\n{err:?}");
                return Some(input.to_owned());
            }
        }
    }
}
