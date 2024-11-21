use anyhow::Result;
use regex::Regex;
use scraper::ElementRef;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pack {
    pub id: String,
    pub title: String,
    pub label: Option<String>,
}

impl Pack {
    pub fn new(element: ElementRef) -> Result<Self> {
        let title = Self::flatten_title(&element.inner_html())?;
        let label = Self::get_label_from_title(&title)?;

        Ok(Self {
            id: element.attr("value").unwrap().to_string(),
            title,
            label,
        })
    }

    fn get_label_from_title(title: &str) -> Result<Option<String>> {
        let reg = Regex::new(r"\[(.*?)\]")?;
        if let Some(captured) = reg.captures(title) {
            if let Some(label) = captured.get(1) {
                return Ok(Some(label.as_str().to_string()));
            }
        }

        Ok(None)
    }

    fn flatten_title(inner_html: &str) -> Result<String> {
        let reg = Regex::new(r"&lt;.*&gt;")?;
        let result = reg.replace_all(inner_html, "").to_string();
        Ok(result)
    }
}

impl fmt::Display for Pack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.id,
            self.title,
            self.label.as_ref().unwrap_or(&"".to_string())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_label_from_title_returns_none() {
        let title = "Linux is dope";
        let label = None;

        let result = Pack::get_label_from_title(title).unwrap();
        assert_eq!(result, label);
    }

    #[test]
    fn get_label_from_title_returns_some() {
        let title = "Linux is dope [Facts]";
        let label = Some("Facts".to_string());

        let result = Pack::get_label_from_title(title).unwrap();
        assert_eq!(result, label);
    }

    #[test]
    fn get_label_from_title_multiple_matches_returns_first() {
        let title = "Linux is dope [Facts] and I love artichokes [Hmm...]";
        let label = Some("Facts".to_string());

        let result = Pack::get_label_from_title(title).unwrap();
        assert_eq!(result, label);
    }

    #[test]
    fn flatten_title_removes_html_tags() {
        let original = "TITLE&lt;br class=\"test\"&gt; - gum is yummy - [1]";
        let flattened = "TITLE - gum is yummy - [1]";

        let result = Pack::flatten_title(original).unwrap();
        assert_eq!(result, flattened);
    }

    #[test]
    fn flatten_title_stays_same() {
        let title = "TITLE - fzf is awesome - [2]";

        let result = Pack::flatten_title(title).unwrap();
        assert_eq!(result, title);
    }
}
