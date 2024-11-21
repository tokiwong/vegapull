use std::time::Instant;

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::Cli;
use localizer::Localizer;
use log::{error, info};

use scraper::OpTcgScraper;

mod card;
mod card_scraper;
mod cli;
mod interactive;
mod localizer;
mod pack;
mod scraper;
mod storage;

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    process_args(args)?;
    Ok(())
}

fn process_args(args: Cli) -> Result<()> {
    let localizer = Localizer::load(args.language)?;
    let scraper = OpTcgScraper::new(&localizer);

    match args.command {
        cli::Commands::Packs => list_packs(&scraper),
        cli::Commands::Cards { pack_id } => list_cards(&scraper, &pack_id.to_string_lossy()),
        cli::Commands::Interactive => interactive::show_interactive(),
    }
}

fn list_packs(scraper: &OpTcgScraper) -> Result<()> {
    info!("fetching all pack ids...");
    let start = Instant::now();

    let packs = scraper.fetch_all_packs()?;
    info!("successfully fetched {} packs!", packs.len());

    let json = serde_json::to_string(&packs)?;
    println!("{}", json);

    let duration = start.elapsed();

    info!("list_packs took: {:?}", duration);
    Ok(())
}

fn list_cards(scraper: &OpTcgScraper, pack_id: &str) -> Result<()> {
    info!("fetching all cards...");
    let start = Instant::now();

    let cards = scraper.fetch_all_cards(&pack_id)?;
    if cards.len() == 0 {
        error!("No cards available for pack `{}`", pack_id);
        return Err(anyhow!("No cards found"));
    }

    info!(
        "successfully fetched {} cards for pack: `{}`!",
        cards.len(),
        pack_id
    );

    let json = serde_json::to_string(&cards)?;
    println!("{}", json);

    let duration = start.elapsed();

    info!("list_cards took: {:?}", duration);
    Ok(())
}
