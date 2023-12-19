use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::{env, fs};

pub fn parse_prompt(mut prompt: String) -> Result<String> {
    let placeholders = get_placeholders(&prompt)?;

    if placeholders.is_empty() {
        return Ok(prompt);
    }

    for placeholder in placeholders {
        let value = get_placeholder_value(&placeholder)?;
        if value == placeholder {
            continue;
        }
        prompt = replace_placeholder(&prompt, &placeholder, &value)?;
    }

    Ok(prompt)
}

fn get_placeholders(text: &str) -> Result<Vec<String>> {
    let regex = Regex::new(r"\{(.*?)}")?;

    let placeholders = regex
        .captures_iter(text)
        .map(|matched| matched.get(1).unwrap().as_str().trim().to_owned())
        .filter(|matched| !matched.is_empty())
        .unique()
        .collect::<Vec<String>>();

    Ok(placeholders)
}

fn get_placeholder_value(key: &str) -> Result<String> {
    let file_regex = Regex::new(r"^\w*(?:\.\w+)+$")?;
    if !file_regex.is_match(&key) {
        return Ok(key.to_string());
    }

    let path = env::current_dir()?.join(&key);
    if !path.exists() {
        return Ok(key.to_string());
    }

    let content = fs::read_to_string(path)?;
    return Ok(content);
}

fn replace_placeholder(text: &str, key: &str, value: &str) -> Result<String> {
    let pattern = regex::escape(&format!("{{{key}}}"));
    let regex = Regex::new(&pattern)?;
    let text = regex.replace_all(&text, value).to_string();

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_placeholders() -> Result<()> {
        let placeholders = get_placeholders("some { placeholder } or {another} also {file.ext }")?;

        assert_eq!(placeholders[0], "placeholder");
        assert_eq!(placeholders[1], "another");
        assert_eq!(placeholders[2], "file.ext");

        assert!(get_placeholders("some {     } or {}")?.is_empty());

        Ok(())
    }
}
