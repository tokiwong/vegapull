use std::fmt;

use regex::Regex;
use scraper::ElementRef;

#[derive(Debug)]
struct BoosterPack {
    pub id: String,
    pub title: String,
    pub reference: Option<String>,
}

impl BoosterPack {
    pub fn new(element: ElementRef) -> BoosterPack {
        let title = BoosterPack::flat_title(&element.inner_html());
        let reference = BoosterPack::ref_from_title(&title);

        BoosterPack {
            id: element.attr("value").unwrap().to_string(),
            title,
            reference,
        }
    }

    pub fn ref_from_title(title: &str) -> Option<String> {
        let reg = Regex::new(r"\[.*\]").unwrap();
        if let Some(captured) = reg.captures_iter(title).next() {
            let reference = captured.get(0).map_or("", |m| m.as_str());
            return Some(reference.to_string());
        }

        None
    }

    pub fn flat_title(inner_html: &str) -> String {
        let reg = Regex::new(r"&lt;.*&gt;").unwrap();
        let result = reg.replace_all(inner_html, "").to_string();
        result
    }
}

impl fmt::Display for BoosterPack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.id,
            self.title,
            self.reference.as_ref().unwrap_or(&"".to_string())
        )
    }
}

fn main() {
    let booster_packs = get_all_booster_packs();

    for bp in booster_packs.iter() {
        println!("{}", bp);
    }
}

fn get_all_booster_packs() -> Vec<BoosterPack> {
    let response = reqwest::blocking::get("https://en.onepiece-cardgame.com/cardlist")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let series_selector = scraper::Selector::parse("div.seriesCol>select#series>option").unwrap();

    let series: Vec<BoosterPack> = document
        .select(&series_selector)
        .map(|x| BoosterPack::new(x))
        .collect();

    series

    // -d "freewords=&series=569201"
}
