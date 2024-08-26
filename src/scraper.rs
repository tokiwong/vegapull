use reqwest::blocking::Client;
use std::collections::HashMap;

use crate::{card::Card, card_set::CardSet};

pub struct OpTcgScraper {
    base_url: String,
}

impl OpTcgScraper {
    pub fn new(base_url: &str) -> OpTcgScraper {
        OpTcgScraper {
            base_url: base_url.to_string(),
        }
    }

    fn cardlist_endpoint(&self) -> String {
        format!("{}/{}", self.base_url, "cardlist")
    }

    pub fn fetch_all_card_sets(&self) -> Vec<CardSet> {
        let response = reqwest::blocking::get(self.cardlist_endpoint())
            .unwrap()
            .text()
            .unwrap();

        let document = scraper::Html::parse_document(&response);
        let series_selector =
            scraper::Selector::parse("div.seriesCol>select#series>option").unwrap();

        let card_sets: Vec<CardSet> = document
            .select(&series_selector)
            .map(|s| CardSet::new(s))
            .collect();

        card_sets

        // -d "freewords=&series=569201"
    }

    pub fn fetch_all_cards(&self, card_set_id: &str) -> Vec<Card> {
        let mut params = HashMap::new();
        params.insert("series", card_set_id);

        let client = Client::new();
        let response = client
            .get(self.cardlist_endpoint())
            .query(&params)
            .send()
            .unwrap()
            .text()
            .unwrap();

        let document = scraper::Html::parse_document(&response);
        let cards_selector = scraper::Selector::parse("div.resultCol>a").unwrap();

        let cards: Vec<Card> = document
            .select(&cards_selector)
            .map(|c| Card::new(&document, c))
            .collect();

        cards
    }
}
