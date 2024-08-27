use anyhow::{anyhow, Context};
use log::info;
use regex::Regex;
use scraper::{ElementRef, Html};

use crate::card::{Card, CardAttribute, CardCategory, CardColor, CardRarity};

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
        let colors = Self::fetch_colors(dl_elem)?;
        let cost = Some(Self::fetch_cost(dl_elem)?);
        let attributes = Self::fetch_attributes(dl_elem)?;
        let power = Self::fetch_power(dl_elem)?;
        let counter = Self::fetch_counter(dl_elem)?;

        let card = Card {
            id,
            name,
            rarity,
            category,
            set_id,
            img_url,
            colors,
            cost,
            attributes,
            power,
            counter,
        };

        Ok(card)
    }

    // element is top level <dl> tag
    pub fn fetch_id(element: ElementRef) -> Result<String, anyhow::Error> {
        let id = element
            .attr("id")
            .context("expected to find id attr on dl")?
            .to_string();

        info!("fetched card_id: {}", id);
        Ok(id)
    }

    pub fn fetch_name(element: ElementRef) -> Result<String, anyhow::Error> {
        let card_name_div = Self::get_child_node(element, "dt>div.cardName".to_string())?;
        let name = card_name_div.inner_html();

        info!("fetched card_name: {}", name);
        Ok(name)
    }

    pub fn fetch_rarity(element: ElementRef) -> Result<CardRarity, anyhow::Error> {
        let rarity_span =
            Self::get_child_node(element, "dt>div.infoCol>span:nth-child(2)".to_string())?;
        let raw_rarity = rarity_span.inner_html();

        info!("fetched card_rarity (raw): {}", raw_rarity);
        let rarity = CardRarity::from_str(&raw_rarity)?;
        Ok(rarity)
    }

    pub fn fetch_category(element: ElementRef) -> Result<CardCategory, anyhow::Error> {
        let category_span =
            Self::get_child_node(element, "dt>div.infoCol>span:nth-child(3)".to_string())?;
        let raw_category = category_span.inner_html();

        info!("fetched card_category (raw): {}", raw_category);
        let category = CardCategory::from_str(&raw_category)?;
        Ok(category)
    }

    pub fn fetch_img_url(element: ElementRef) -> Result<String, anyhow::Error> {
        let img = Self::get_child_node(element, "dd>div.frontCol>img".to_string())?;
        let img_url = img
            .attr("data-src")
            .context("no data-src attr")?
            .to_string();

        info!("fetched card_img_url: {}", img_url);
        Ok(img_url)
    }

    fn strip_html_tags(value: &str) -> Result<String, anyhow::Error> {
        let reg = Regex::new(r"<[^>]*>.*?</[^>]*>")?;
        let result = reg.replace_all(&value, "").trim().to_string();
        Ok(result)
    }

    pub fn fetch_colors(element: ElementRef) -> Result<Vec<CardColor>, anyhow::Error> {
        let color_div = Self::get_child_node(element, "dd>div.backCol>div.color".to_string())?;
        let content = color_div.inner_html();

        let raw_color_list = Self::strip_html_tags(&content)?;
        info!("fetched card_colors_list: {}", raw_color_list);

        let raw_colors: Vec<&str> = raw_color_list.split('/').collect();

        let mut colors = Vec::new();
        for raw_color in raw_colors.iter() {
            info!("processing card_color: {}", raw_color);
            let color = CardColor::from_str(&raw_color)?;
            colors.push(color);
        }

        Ok(colors)
    }

    pub fn fetch_cost(element: ElementRef) -> Result<i32, anyhow::Error> {
        let cost_div =
            Self::get_child_node(element, "dd>div.backCol>div.col2>div.cost".to_string())?;
        let content = cost_div.inner_html();

        let raw_cost = Self::strip_html_tags(&content)?;
        info!("fetched card_cost (raw): {}", raw_cost);

        let cost: i32 = raw_cost.parse()?;
        Ok(cost)
    }

    pub fn fetch_attributes(element: ElementRef) -> Result<Vec<CardAttribute>, anyhow::Error> {
        let attr_div = Self::get_child_node(
            element,
            "dd>div.backCol>div.col2>div.attribute>i".to_string(),
        )?;
        let raw_attr = attr_div.inner_html();
        info!("fetched card_attribute (raw): {}", raw_attr);
        if raw_attr.is_empty() {
            return Ok(Vec::new());
        }

        let attribute = CardAttribute::from_str(&raw_attr)?;
        Ok(vec![attribute])
    }

    pub fn fetch_power(element: ElementRef) -> Result<Option<i32>, anyhow::Error> {
        let power_div =
            Self::get_child_node(element, "dd>div.backCol>div.col2>div.power".to_string())?;
        let content = power_div.inner_html();

        let raw_power = Self::strip_html_tags(&content)?;
        info!("fetched card_power (raw): {}", raw_power);
        if raw_power == "-" {
            return Ok(None);
        }

        let power: i32 = raw_power.parse()?;
        Ok(Some(power))
    }

    pub fn fetch_counter(element: ElementRef) -> Result<Option<i32>, anyhow::Error> {
        let counter_div =
            Self::get_child_node(element, "dd>div.backCol>div.col2>div.counter".to_string())?;
        let content = counter_div.inner_html();

        let raw_counter = Self::strip_html_tags(&content)?;
        info!("fetched card_power (raw): {}", raw_counter);

        if raw_counter == "-" {
            return Ok(None);
        }

        let counter: i32 = raw_counter.parse()?;
        Ok(Some(counter))
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
        let dl_sel = scraper::Selector::parse(&dl_sel).unwrap();
        let dl_elem = document.select(&dl_sel).next().unwrap();

        Ok(dl_elem)
    }
}
