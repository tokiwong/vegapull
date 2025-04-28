use std::{env, fs, path::Path, process::ExitCode, time::Instant};

use anyhow::{bail, Result};
use clap::Parser;
use cli::{Cli, LanguageCode};
use localizer::Localizer;
use log::{error, info};

use scraper::OpTcgScraper;
use storage::DataStore;

mod card;
mod cli;
mod interactive;
mod localizer;
mod pack;
mod scraper;
mod storage;

fn main() -> ExitCode {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    match process_args(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{}", e);
            ExitCode::FAILURE
        }
    }
}

fn process_args(args: Cli) -> Result<()> {
    let default_config_dir = env::current_dir()?.join("config");
    let config_dir = args.config_directory_path.unwrap_or(default_config_dir);
    info!("using configuration from: {}", config_dir.display());

    match args.command {
        cli::Commands::Packs => list_packs(&config_dir, args.language),
        cli::Commands::Cards { pack_id } => {
            list_cards(&config_dir, args.language, &pack_id.to_string_lossy())
        }
        cli::Commands::Interactive => interactive::show_interactive(&config_dir),
        cli::Commands::Images {
            pack_id,
            output_dir,
        } => download_images(
            &config_dir,
            args.language,
            &pack_id.to_string_lossy(),
            &output_dir,
        ),
        cli::Commands::TestConfig => Localizer::find_locales(&config_dir),
    }
}

fn download_images(
    config_dir: &Path,
    language: LanguageCode,
    pack_id: &str,
    output_dir: &Path,
) -> Result<()> {
    let localizer = Localizer::load(config_dir, language)?;
    let scraper = OpTcgScraper::new(&localizer);

    if output_dir.exists() {
        error!("output directory already `{}` exists", output_dir.display());
        bail!(
            "cannot create directory `{}` to store images because it already exists",
            output_dir.display()
        );
    }

    match fs::create_dir_all(output_dir) {
        Ok(_) => info!("successfully created `{}`", output_dir.display()),
        Err(e) => bail!("failed to create `{}`: {}", output_dir.display(), e),
    }

    info!("fetching all cards for pack `{}`...", pack_id);
    let start = Instant::now();

    let cards = scraper.fetch_all_cards(pack_id)?;
    if cards.is_empty() {
        error!("no cards available for pack `{}`", pack_id);
        bail!("no cards found for pack `{}`", pack_id);
    }

    info!(
        "successfully fetched {} cards for pack: `{}`!",
        cards.len(),
        pack_id
    );

    let duration = start.elapsed();
    info!("fetching cards took: {:?}", duration);

    info!("downloading images for pack `{}`...", pack_id);
    let start = Instant::now();

    for (idx, card) in cards.iter().enumerate() {
        let img_filename = DataStore::get_img_filename(card)?;
        let img_path = output_dir.join(img_filename);

        let img_data = scraper.download_card_image(card)?;
        DataStore::write_image_to_file(img_data, &img_path)?;

        info!(
            "[{}/{}] saved image `{}` to `{}`",
            idx + 1,
            cards.len(),
            card.img_url,
            img_path.display()
        );
    }

    let duration = start.elapsed();
    info!("downloading images took: {:?}", duration);
    Ok(())
}

fn list_packs(config_dir: &Path, language: LanguageCode) -> Result<()> {
    let localizer = Localizer::load(config_dir, language)?;
    let scraper = OpTcgScraper::new(&localizer);

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

fn list_cards(config_dir: &Path, language: LanguageCode, pack_id: &str) -> Result<()> {
    let localizer = Localizer::load(config_dir, language)?;
    let scraper = OpTcgScraper::new(&localizer);

    info!("fetching all cards...");
    let start = Instant::now();

    let cards = scraper.fetch_all_cards(pack_id)?;
    if cards.is_empty() {
        error!("No cards available for pack `{}`", pack_id);
        bail!("No cards found");
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
