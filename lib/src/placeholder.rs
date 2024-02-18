use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{fs, io, path};

const PLACEHOLDER_KEY_PATTERN: &str = r"\{([^{]*?)}";
const FILE_PATTERN: &str = r"^.*(?:\.\w+)+$";
const FILE_SLICE_PATTERN: &str = r"^(?P<path>.*(?:\.\w+)+) *(?P<from>\d*)*?:(?P<to>\d*)*?$";

#[derive(Debug)]
enum Placeholder {
    File(FilePlaceholder),
    FileSlice(FileSlicePlaceholder),
    Unknown(String),
}

#[derive(Debug)]
struct FilePlaceholder {
    file_path: String,
}

#[derive(Debug)]
struct FileSlicePlaceholder {
    key: String,
    file_path: String,
    from_line: usize,
    to_line: usize,
}

impl FilePlaceholder {
    pub fn value(&self) -> Result<String> {
        let path = path::absolute(&self.file_path)?;
        if !path.exists() {
            eprintln!("file {path:?} does not exist");
            return Ok(self.file_path.to_owned());
        }

        let content = fs::read_to_string(path)?;
        Ok(content)
    }
}

impl From<String> for FilePlaceholder {
    fn from(key: String) -> Self {
        Self { file_path: key }
    }
}

impl FileSlicePlaceholder {
    pub fn value(&self) -> Result<String> {
        let path = path::absolute(&self.file_path)?;
        if !path.exists() {
            eprintln!("file {path:?} does not exist");
            return Ok(self.file_path.to_owned());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let content_sliced = reader
            .lines()
            .skip(self.from_line - 1)
            .take(self.to_line - self.from_line + 1)
            .collect::<io::Result<Vec<String>>>()?
            .join("\n");

        Ok(content_sliced)
    }
}

impl From<String> for FileSlicePlaceholder {
    fn from(key: String) -> Self {
        let captures = Regex::new(FILE_SLICE_PATTERN)
            .unwrap()
            .captures(&key)
            .unwrap();

        let file_path = match captures.name("path") {
            None => "".to_owned(),
            Some(matched) => matched.as_str().to_owned(),
        };

        let from_line = match captures.name("from") {
            None => 1,
            Some(matched) => matched.as_str().parse::<usize>().unwrap_or(1),
        };

        let to_line = match captures.name("to") {
            None => usize::MAX,
            Some(matched) => matched.as_str().parse::<usize>().unwrap_or(usize::MAX),
        };

        Self {
            key,
            file_path,
            from_line,
            to_line,
        }
    }
}

impl Placeholder {
    pub fn value(self) -> Result<String> {
        match self {
            Placeholder::File(file) => file.value(),
            Placeholder::FileSlice(file_slice) => file_slice.value(),
            Placeholder::Unknown(key) => Ok(key),
        }
    }

    pub fn key(&self) -> &str {
        match self {
            Placeholder::File(file) => &file.file_path,
            Placeholder::FileSlice(file_slice) => &file_slice.key,
            Placeholder::Unknown(key) => key,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(*self, Placeholder::Unknown(_))
    }
}

impl From<String> for Placeholder {
    fn from(key: String) -> Self {
        if is_file_slice(&key) {
            return Placeholder::FileSlice(FileSlicePlaceholder::from(key));
        }
        if is_file_path(&key) {
            return Placeholder::File(FilePlaceholder::from(key));
        }

        Placeholder::Unknown(key)
    }
}

pub fn replace_placeholders(mut text: String) -> Result<String> {
    let placeholder_keys = get_placeholder_keys(&text);
    if placeholder_keys.is_empty() {
        return Ok(text);
    }

    for key in placeholder_keys {
        text = replace_placeholder(&text, Placeholder::from(key))?;
    }

    Ok(text)
}

fn get_placeholder_keys(text: &str) -> Vec<String> {
    let regex = Regex::new(PLACEHOLDER_KEY_PATTERN).unwrap();
    regex
        .captures_iter(text)
        .map(|matched| matched[1].trim().to_owned())
        .filter(|matched| !matched.is_empty())
        .unique()
        .collect()
}

fn replace_placeholder(text: &str, placeholder: Placeholder) -> Result<String> {
    if placeholder.is_unknown() {
        return Ok(text.to_string());
    }

    let regex = Regex::new(&regex::escape(&format!("{{{}}}", placeholder.key())))?;
    let text = regex.replace_all(&text, placeholder.value()?).to_string();

    Ok(text)
}

fn is_file_path(key: &str) -> bool {
    Regex::new(FILE_PATTERN).unwrap().is_match(&key)
}

fn is_file_slice(key: &str) -> bool {
    Regex::new(FILE_SLICE_PATTERN).unwrap().is_match(&key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_placeholders() -> Result<()> {
        let placeholder_keys = get_placeholder_keys(
            "some { placeholder } or {another} {file.ext } {{ file name.ext}} {file name.ext 10:20}",
        );

        assert_eq!(placeholder_keys[0], "placeholder");
        assert_eq!(placeholder_keys[1], "another");
        assert_eq!(placeholder_keys[2], "file.ext");
        assert_eq!(placeholder_keys[3], "file name.ext");
        assert_eq!(placeholder_keys[4], "file name.ext 10:20");

        assert!(get_placeholder_keys("some {     } or {}").is_empty());

        Ok(())
    }

    #[test]
    fn test_is_file_path() -> Result<()> {
        assert!(!is_file_path("unknown"));
        assert!(!is_file_path("un known"));
        assert!(is_file_path("file.ext"));
        assert!(is_file_path("na me.ext"));
        assert!(is_file_path(r".\na me.ext"));
        assert!(is_file_path(r"..\na me.ext"));
        assert!(is_file_path(r"..\..\na me.ext"));
        assert!(is_file_path(r"home/usr/my_project/docker-compose.yml"));

        Ok(())
    }

    #[test]
    fn test_is_file_slice() -> Result<()> {
        assert!(!is_file_slice("na me.ext"));
        assert!(!is_file_slice("1:20"));
        assert!(!is_file_slice(":20"));
        assert!(!is_file_slice("   10:20"));
        assert!(is_file_slice("name.ext 10:20"));
        assert!(is_file_slice("name.ext    10:20"));
        assert!(is_file_slice("name.ext :5"));
        assert!(is_file_slice("name.ext 5:"));
        assert!(is_file_slice(r"..\..\na me.ext 10:20"));

        Ok(())
    }
}
