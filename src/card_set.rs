use regex::Regex;
use scraper::ElementRef;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct CardSet {
    pub id: String,

    pub title: String,
    pub set_id: Option<String>,
}

impl CardSet {
    pub fn new(element: ElementRef) -> CardSet {
        let title = CardSet::flat_title(&element.inner_html());
        let reference = CardSet::ref_from_title(&title);

        CardSet {
            id: element.attr("value").unwrap().to_string(),
            title,
            set_id: reference,
        }
    }

    fn ref_from_title(title: &str) -> Option<String> {
        let reg = Regex::new(r"\[.*\]").unwrap();
        if let Some(captured) = reg.captures_iter(title).next() {
            let reference = captured.get(0).map_or("", |m| m.as_str());
            return Some(reference.to_string());
        }

        None
    }

    fn flat_title(inner_html: &str) -> String {
        let reg = Regex::new(r"&lt;.*&gt;").unwrap();
        let result = reg.replace_all(inner_html, "").to_string();
        result
    }
}

impl fmt::Display for CardSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.id,
            self.title,
            self.set_id.as_ref().unwrap_or(&"".to_string())
        )
    }
}
