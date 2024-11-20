use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{anyhow, bail, Result};
use clap::{Parser, ValueEnum};
use cli::{Cli, LanguageCode};
use localizer::Localizer;
use log::{error, info};

use op_data::DataStore;
use op_scraper::OpTcgScraper;

mod card;
mod card_scraper;
mod cli;
mod localizer;
mod op_data;
mod op_scraper;
mod pack;

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
        cli::Commands::Interactive => show_interactive(),
    }
}

fn show_interactive() -> Result<()> {
    println!("+---------------------------+");
    println!("| VegaPull Interactive Mode |");
    println!("+---------------------------+\n");

    info!("prompting user for language selection");
    let input = input_prompt("Enter language (english): ")?;
    info!("user input: {}", input);

    let value = input.is_empty().then(|| "english").unwrap_or(&input);
    info!("value to use: {}", value);

    let language = match LanguageCode::from_str(&value, true) {
        Ok(val) => val,
        Err(_) => bail!("Failed to parse language code, invalid value: {}", value),
    };
    info!("using language: {:?}", language);

    info!("prompting user for save location");
    let input = input_prompt("Enter location to save data (./data): ")?;
    info!("user input: {}", input);

    let value = input.is_empty().then(|| "./data").unwrap_or(&input);
    info!("value to use: {}", value);

    let data_dir = PathBuf::from(&value);
    if data_dir.exists() {
        info!(
            "directory `{}` exists, prompting user for removal",
            data_dir.display()
        );

        let prompt = format!(
            "Dir `{}` already exists. Replace? (y/N): ",
            data_dir.display()
        );
        let input = input_prompt(&prompt)?;
        info!("user input: {}", input);

        let value = input.is_empty().then(|| "no").unwrap_or(&input);
        info!("value to use: {}", value);

        if is_yes(&value) {
            println!("Cleared directory: `{}`", data_dir.display());
            fs::remove_dir_all(&data_dir)?;
            info!("removed directory: {}", data_dir.display());
        } else {
            info!("user cancelled directory removal: {}", data_dir.display());
            bail!("Aborted, directory has been kept: `{}`", data_dir.display());
        }
    }

    info!("prompting user whether to download images");
    let input = input_prompt("Download images as well (y/N): ")?;
    info!("user input: {}", input);

    let value = input.is_empty().then(|| "no").unwrap_or(&input);
    info!("value to use: {}", value);

    let download_images = is_yes(&value);

    let localizer = Localizer::load(language)?;
    let scraper = OpTcgScraper::new(&localizer);
    let store = DataStore::new(&data_dir, language);

    println!("\nFetching packs...");
    let start = Instant::now();

    let packs = scraper.fetch_all_packs()?;
    store.write_packs(&packs)?;

    let duration = start.elapsed();
    info!("fetching packs took: {:?}", duration);

    println!("Successfully stored data for {} packs!\n", packs.len());

    let start = Instant::now();
    for (idx, pack) in packs.iter().enumerate() {
        print!(
            "[{}/{}] Fetching cards for pack `{}`...",
            (idx + 1),
            packs.len(),
            pack.id
        );
        io::stdout().flush()?;

        let cards = scraper.fetch_all_cards(&pack.id)?;
        if cards.len() == 0 {
            error!("no cards available for pack `{}`", &pack.id);
            bail!("No cards found");
        }

        store.write_cards(&pack.id, &cards)?;
        info!("fetched and wrote cards for: `{}`", pack.id);
        println!(" OK");

        if download_images {
            println!("Downloading images for pack `{}`...", pack.id);
            for (idx, card) in cards.iter().enumerate() {
                print!(
                    "[{}/{}] Downloading image for card `{}`...",
                    idx + 1,
                    cards.len(),
                    card.id
                );
                io::stdout().flush()?;

                let img_data = scraper.download_card_image(&card)?;
                store.write_image(&card, img_data)?;
                println!(" OK");
            }
        }
    }

    let duration = start.elapsed();
    info!("fetching cards (and images) took: {:?}", duration);

    println!("Final data is available in: {}", data_dir.display());

    Ok(())
}

fn is_yes(input: &str) -> bool {
    let input = input.trim().to_lowercase();
    matches!(input.as_str(), "yes" | "y")
}

fn input_prompt(text: &str) -> Result<String> {
    print!("{}", text);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
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
