use anyhow::Result;
use data::{write_cards, write_sets};
use log::info;
use scraper::OpTcgScraper;

mod card;
mod card_scraper;
mod card_set;
mod data;
mod scraper;

fn main() -> Result<()> {
    env_logger::init();

    info!("start OP TCG Scraper");

    let host = "https://en.onepiece-cardgame.com";
    let scraper = OpTcgScraper::new(host);

    // println!("Fetching all card sets");
    // let card_sets = scraper.fetch_all_card_sets()?;
    // write_sets(&card_sets)?;
    //
    // println!("Successfully fetched {} card sets", card_sets.len());

    let set_id = "569001";

    println!("Fetching all cards for set: {}", set_id);
    let cards = scraper.fetch_all_cards(set_id)?;
    write_cards(&cards, set_id)?;

    // for card_set in card_sets.iter() {
    //     println!("Fetching all cards for set: {}", card_set);
    //     let cards = scraper.fetch_all_cards(&card_set.id)?;
    //     write_cards(&cards, &card_set.id)?;
    //
    //     println!(
    //         "Successfully fetched {} cards for set: {}",
    //         cards.len(),
    //         card_set
    //     );
    // }

    Ok(())
}
