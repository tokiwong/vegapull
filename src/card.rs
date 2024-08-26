use anyhow::anyhow;
use scraper::{ElementRef, Html};

#[derive(Debug)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub rarity: CardRarity,
    pub category: CardCategory,
    // pub number: i32,
    pub set_id: String,
    // pub copyright: String,

    // Images
    // pub thumb_url: String,
    pub img_url: String,
    // pub illustration: CardIllustration,
    // pub illustrator_name: String,
    // Gameplay
    // pub colors: Vec<CardColor>,
    // pub life: Option<i32>,              // Only Leader
    // pub cost: Option<i32>,              // Only Character, Event and Stage
    // pub attributes: Vec<CardAttribute>, // Only Leader and Character
    // pub power: Option<i32>,             // Only Leader and Character
    // pub counter: Option<i32>,           // Only Character
    //
    // pub types: Vec<String>,
    // pub effect: String,
    // pub trigger: Option<String>,
    // pub notes: String,
}

#[derive(Debug)]
pub enum CardColor {
    Red,
    Green,
    Blue,
    Purple,
    Black,
    Yellow,
}

#[derive(Debug)]
pub enum CardIllustration {
    Comic,
    Animation,
    Original,
    Other,
}

#[derive(Debug)]
pub enum CardAttribute {
    Slash,
    Strike,
    Ranged,
    Special,
    Wisdom,
}

#[derive(Debug)]
pub enum CardCategory {
    Leader,
    Character,
    Event,
    Stage,
    Don,
}

impl CardCategory {
    pub fn from_str(value: &str) -> Result<CardCategory, anyhow::Error> {
        match value.to_uppercase().as_str() {
            "LEADER" => Ok(Self::Leader),
            "CHARACTER" => Ok(Self::Character),
            "EVENT" => Ok(Self::Event),
            "STAGE" => Ok(Self::Stage),
            "DOn" => Ok(Self::Don),
            _ => Err(anyhow!("Unsupported category `{}`", value)),
        }
    }
}

#[derive(Debug)]
pub enum CardRarity {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    SuperRare = 3,
    SecretRare = 4,
    Leader = 5,
    Special = 6,
}

impl CardRarity {
    pub fn from_str(value: &str) -> Result<CardRarity, anyhow::Error> {
        match value.to_uppercase().as_str() {
            "C" => Ok(Self::Common),
            "UC" => Ok(Self::Uncommon),
            "R" => Ok(Self::Rare),
            "SR" => Ok(Self::SuperRare),
            "SEC" => Ok(Self::SecretRare),
            "L" => Ok(Self::Leader),
            "SP CARD" => Ok(Self::Special),
            _ => Err(anyhow!("Unsupported rarity `{}`", value)),
        }
    }
}
