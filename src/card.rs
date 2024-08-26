use scraper::{ElementRef, Html};

#[derive(Debug)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub img_url: String,

    pub rarity: CardRarity,
    pub card_type: CardType,
    // pub details: CardDetails,
    // pub cost: i32,
    // pub attribute: String,
    // pub power: i32,
    // pub counter: Option<String>,
    // pub color: String,
    //
    // pub effect: String,
    //
    // pub pack_id: String,
}

impl Card {
    pub fn new(document: &Html, element: ElementRef) -> Card {
        // let img_sel = scraper::Selector::parse("img").unwrap();
        // let img_element = element.select(&img_sel).next().unwrap();
        let id = element.attr("data-src").unwrap().to_string();

        let id = &id[1..];

        let dl_elm = Card::get_desc(document, id);
        let dt_elm = Card::get_desc_term(dl_elm);
        let dd_elm = Card::get_desc_term(dl_elm);

        Card {
            id: id.to_string(),
            img_url: Card::get_url(element),
            name: Card::get_card_name(dt_elm),

            rarity: CardRarity::Common,
            card_type: CardType::Don,
        }
    }

    fn get_url(element: ElementRef) -> String {
        let img_sel = scraper::Selector::parse("img").unwrap();
        element
            .select(&img_sel)
            .next()
            .unwrap()
            .attr("src")
            .unwrap()
            .to_string()
    }

    fn get_rarity(dt_elm: ElementRef) -> CardRarity {
        let info_col_sel = scraper::Selector::parse("div.infoCol>span").unwrap();
        let info_col_divs = dt_elm.select(&info_col_sel);

        // for info_span in dt_elm.select(&info_col_sel) {}

        CardRarity::Common
    }

    fn get_card_name(dt_elm: ElementRef) -> String {
        let card_name_sel = scraper::Selector::parse("div.cardName").unwrap();
        dt_elm.select(&card_name_sel).next().unwrap().inner_html()
    }

    fn get_desc<'a>(document: &'a Html, card_id: &'a str) -> ElementRef<'a> {
        let dl_sel = format!("dl.{}", card_id);
        let dl_sel = scraper::Selector::parse(&dl_sel).unwrap();
        let dl_elm = document.select(&dl_sel).next().unwrap();
        dl_elm
    }

    fn get_desc_term(desc_elm: ElementRef) -> ElementRef {
        let dt_sel = scraper::Selector::parse("dt").unwrap();
        let dt_elm = desc_elm.select(&dt_sel).next().unwrap();
        dt_elm
    }

    fn get_desc_details(desc_elm: ElementRef) -> ElementRef {
        let dd_sel = scraper::Selector::parse("dd").unwrap();
        let dd_elm = desc_elm.select(&dd_sel).next().unwrap();
        dd_elm
    }
}

#[derive(Debug)]
pub enum CardType {
    Leader,
    Character,
    Event,
    Stage,
    Don,
}

#[derive(Debug)]
pub enum CardRarity {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    SuperRare = 3,
    SecretRare = 4,
    Leader = 5,
}
