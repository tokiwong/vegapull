use anyhow::Context;
use reqwest::blocking::Client;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{card::Card, card_scraper::CardScraper, card_set::CardSet};

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

    pub fn fetch_all_card_sets(&self, save_to_file: bool) -> Result<Vec<CardSet>, anyhow::Error> {
        let response = reqwest::blocking::get(self.cardlist_endpoint())?.text()?;

        let document = scraper::Html::parse_document(&response);
        let series_selector =
            scraper::Selector::parse("div.seriesCol>select#series>option").unwrap();

        let card_sets: Vec<CardSet> = document
            .select(&series_selector)
            .map(|x| CardSet::new(x))
            .filter(|cs| cs.id != "")
            .collect();

        if save_to_file {
            Self::write_sets_to_file(&card_sets)?;
        }

        Ok(card_sets)
        // -d "freewords=&series=569201"
    }

    fn card_sets_file_path() -> PathBuf {
        PathBuf::from("data/card_sets.json")
    }

    fn write_sets_to_file(card_sets: &Vec<CardSet>) -> Result<(), anyhow::Error> {
        let file_path = Self::card_sets_file_path();
        // info!("write groups to file: `{}`", file_path.to_string_lossy());

        let json = serde_json::to_string(card_sets)?;
        // debug!("serialize groups: `{:?} -> {}`", groups, json);

        fs::write(file_path, json)?;

        Ok(())
    }

    pub fn fetch_all_cards(&self, card_set_id: &str) -> Result<Vec<Card>, anyhow::Error> {
        let mut params = HashMap::new();
        params.insert("series", card_set_id);

        let client = Client::new();
        let response = client
            .get(self.cardlist_endpoint())
            .query(&params)
            .send()?
            .text()?;

        let document = scraper::Html::parse_document(&response);
        let card_ids_sel = scraper::Selector::parse("div.resultCol>a").unwrap();

        let mut cards = Vec::new();
        for element in document.select(&card_ids_sel) {
            let card_id = element
                .attr("data-src")
                .context("expected `data-src` attr on <a>")?
                .to_string();

            let card_id = &card_id[1..];

            let card = CardScraper::create_card(&document, card_set_id, &card_id)?;
            cards.push(card);
        }

        Ok(cards)
    }
}
