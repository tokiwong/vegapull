use scraper::OpTcgScraper;

mod card;
mod card_set;
mod scraper;

fn main() {
    let scraper = OpTcgScraper::new("https://en.onepiece-cardgame.com");
    let card_sets = scraper.fetch_all_card_sets();

    for card_set in card_sets.iter() {
        println!("{}", card_set);
    }

    // let cards = get_cards("569201");
    // for card in cards.iter() {
    //     println!("{:?}", card);
    // }
}
