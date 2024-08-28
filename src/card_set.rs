use regex::Regex;
use scraper::ElementRef;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::card::Card;

#[derive(Debug, Deserialize, Serialize)]
pub struct CardSet {
    pub id: String,
    pub title: String,
    pub label: Option<String>,

    pub cards: Vec<Card>,
}

impl CardSet {
    pub fn new(element: ElementRef) -> CardSet {
        let title = CardSet::flatten_title(&element.inner_html());
        let label = CardSet::get_label_from_title(&title);

        CardSet {
            id: element.attr("value").unwrap().to_string(),
            title,
            label,
            cards: Vec::new(),
        }
    }

    fn get_label_from_title(title: &str) -> Option<String> {
        let reg = Regex::new(r"\[.*\]").unwrap();
        if let Some(captured) = reg.captures_iter(title).next() {
            let label = captured.get(0).map_or("", |m| m.as_str());
            return Some(label.to_string());
        }

        None
    }

    fn flatten_title(inner_html: &str) -> String {
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
            self.label.as_ref().unwrap_or(&"".to_string())
        )
    }
}
