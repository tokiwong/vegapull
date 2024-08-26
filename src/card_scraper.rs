use anyhow::{anyhow, Context};
use scraper::{ElementRef, Html};

use crate::card::{Card, CardCategory, CardRarity};

pub struct CardScraper {}

impl CardScraper {
    pub fn create_card(
        document: &Html,
        set_id: &str,
        card_id: &str,
    ) -> Result<Card, anyhow::Error> {
        let dl_elem = Self::get_dl_node(document, card_id.to_string())?;

        let id = Self::fetch_id(dl_elem)?;
        let name = Self::fetch_name(dl_elem)?;
        let rarity = Self::fetch_rarity(dl_elem)?;
        let category = Self::fetch_category(dl_elem)?;
        let set_id = set_id.to_string();
        let img_url = Self::fetch_img_url(dl_elem)?;

        let card = Card {
            id,
            name,
            rarity,
            category,
            set_id,
            img_url,
        };

        Ok(card)
    }

    // element is top level <dl> tag
    pub fn fetch_id(element: ElementRef) -> Result<String, anyhow::Error> {
        println!("fetch_id");
        Ok(element
            .attr("id")
            .context("expected to find id attr on dl")?
            .to_string())
    }

    pub fn fetch_name(element: ElementRef) -> Result<String, anyhow::Error> {
        println!("fetch_name");
        let card_name_div = Self::get_child_node(element, "dt>div.cardName".to_string())?;
        Ok(card_name_div.inner_html())
    }

    pub fn fetch_rarity(element: ElementRef) -> Result<CardRarity, anyhow::Error> {
        let rarity_span =
            Self::get_child_node(element, "dt>div.infoCol>span:nth-child(2)".to_string())?;
        let raw_rarity = rarity_span.inner_html();
        let rarity = CardRarity::from_str(&raw_rarity)?;
        Ok(rarity)
    }

    pub fn fetch_category(element: ElementRef) -> Result<CardCategory, anyhow::Error> {
        println!("fetch_category");
        let category_span =
            Self::get_child_node(element, "dt>div.infoCol>span:nth-child(3)".to_string())?;
        let raw_category = category_span.inner_html();
        let category = CardCategory::from_str(&raw_category)?;
        Ok(category)
    }

    pub fn fetch_img_url(element: ElementRef) -> Result<String, anyhow::Error> {
        println!("fetch_img_url");
        let img = Self::get_child_node(element, "dd>div.frontCol>img".to_string())?;
        let img_url = img
            .attr("data-src")
            .context("no data-src attr")?
            .to_string();

        Ok(img_url)
    }

    fn get_child_node(element: ElementRef, selector: String) -> Result<ElementRef, anyhow::Error> {
        let node_sel = scraper::Selector::parse(&selector).unwrap();
        let results: Vec<_> = element.select(&node_sel).collect();

        match results.len() {
            0 => Err(anyhow!("Expected `{}` but got nothing", selector)),
            1 => Ok(*results.iter().next().unwrap()),
            _ => Err(anyhow!("Expected single `{}` but got many", selector)),
        }
    }

    pub fn get_dl_node(document: &Html, card_id: String) -> Result<ElementRef, anyhow::Error> {
        let dl_sel = format!("dl#{}", card_id);
        println!("{}", dl_sel);
        let dl_sel = scraper::Selector::parse(&dl_sel).unwrap();
        let dl_elem = document.select(&dl_sel).next().unwrap();

        Ok(dl_elem)
    }
}
