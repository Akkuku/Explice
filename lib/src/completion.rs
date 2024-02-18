use anyhow::Context;
use regex::Regex;
use std::path::PathBuf;
use std::{env, fs, io, path};

const INCOMPLETE_FILE_PATH_PATTERN: &str =
    r"(?P<rest>.*\{)(?P<complete_path>[^}\n]+[\/\\])?(?P<incomplete_path>[^}\n]+)?$";

#[derive(Debug)]
struct IncompleteFilePath {
    complete_path: Option<String>,
    incomplete_path: Option<String>,
    input_before: String,
}

impl TryFrom<&str> for IncompleteFilePath {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let captures = Regex::new(INCOMPLETE_FILE_PATH_PATTERN)?
            .captures(input)
            .context("not found incomplete path")?;

        let complete_path = match captures.name("complete_path") {
            None => None,
            Some(matched) => Some(matched.as_str().to_owned()),
        };

        let incomplete_path = match captures.name("incomplete_path") {
            None => None,
            Some(matched) => Some(matched.as_str().to_owned()),
        };

        let rest = match captures.name("rest") {
            None => None,
            Some(matched) => Some(matched.as_str().to_owned()),
        };

        Ok(Self {
            complete_path,
            incomplete_path,
            input_before: rest.unwrap_or_default(),
        })
    }
}

pub fn get_path_completion(input: &str) -> anyhow::Result<String> {
    if input.is_empty() || !has_incomplete_path(input) {
        return Ok(input.to_owned());
    }

    let path = IncompleteFilePath::try_from(input)?;
    let incomplete_path_name = path.incomplete_path.as_deref().unwrap_or_default();

    let entries = get_directory_entry_paths(path.complete_path.as_ref())?;
    let matching_entry_names: Vec<String> = entries
        .into_iter()
        .filter_map(|entry| match entry.file_name() {
            Some(file_name) => Some((
                file_name.to_str().unwrap_or_default().to_owned(),
                entry.is_dir(),
            )),
            None => None,
        })
        .filter(|(file_name, _)| file_name.starts_with(incomplete_path_name))
        .map(|(file_name, is_dir)| {
            if is_dir {
                format!("{file_name}/")
            } else {
                file_name
            }
        })
        .collect();

    if matching_entry_names.is_empty() {
        return Ok(input.to_owned());
    }

    let completion = format!(
        "{}{}{}",
        path.input_before,
        path.complete_path.as_deref().unwrap_or_default(),
        &matching_entry_names[0]
    );

    Ok(completion)
}

fn has_incomplete_path(input: &str) -> bool {
    Regex::new(INCOMPLETE_FILE_PATH_PATTERN)
        .unwrap()
        .is_match(&input)
}

fn get_directory_entry_paths(path: Option<&String>) -> io::Result<Vec<PathBuf>> {
    let current_path = match path {
        None => env::current_dir()?,
        Some(path) => path::absolute(path)?,
    };

    let entry_paths = fs::read_dir(current_path)?
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    Ok(entry_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_completion() -> anyhow::Result<()> {
        assert_eq!(get_path_completion("{Carg")?, "{Cargo.toml".to_owned());
        assert_eq!(
            get_path_completion("some text {src")?,
            "some text {src/".to_owned()
        );
        assert_eq!(
            get_path_completion("some text {src/comp")?,
            "some text {src/completion.rs".to_owned()
        );

        Ok(())
    }
}
