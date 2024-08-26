use anyhow::Result;
use log::info;
use scraper::OpTcgScraper;

mod card;
mod card_scraper;
mod card_set;
mod scraper;

fn main() -> Result<()> {
    env_logger::init();

    info!("start OP TCG Scraper");

    let host = "https://en.onepiece-cardgame.com";
    let scraper = OpTcgScraper::new(host);

    // let card_sets = scraper.fetch_all_card_sets(true)?;
    //
    // for card_set in card_sets.iter() {
    //     println!("{}", card_set);
    // }
    //

    let cards = scraper.fetch_all_cards("569201")?;
    for card in cards.iter() {
        println!("{:?}", card);
    }

    Ok(())
}
