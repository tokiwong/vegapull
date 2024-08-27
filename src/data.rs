use log::{debug, info};
use std::{fs, path::PathBuf};

use crate::{card::Card, card_set::CardSet};

fn data_dir_path() -> PathBuf {
    PathBuf::from("data/")
}

fn card_sets_file_path() -> PathBuf {
    data_dir_path().join("sets.json")
}

fn cards_file_path(set_id: &str) -> PathBuf {
    let filename = format!("cards_{}.json", set_id);
    data_dir_path().join(filename)
}

pub fn write_sets(card_sets: &Vec<CardSet>) -> Result<(), anyhow::Error> {
    let file_path = card_sets_file_path();

    info!(
        "write list of {} card_sets to file: `{}`",
        card_sets.len(),
        file_path.to_string_lossy()
    );

    let json = serde_json::to_string(card_sets)?;
    debug!("serialize card_sets: `{:?} -> {}`", card_sets, json);

    fs::write(file_path, json)?;

    Ok(())
}

pub fn write_cards(cards: &Vec<Card>, set_id: &str) -> Result<(), anyhow::Error> {
    let file_path = cards_file_path(set_id);

    info!(
        "write list of {} cards from set {} to file: `{}`",
        cards.len(),
        set_id,
        file_path.to_string_lossy()
    );

    let json = serde_json::to_string(cards)?;
    debug!("serialize cards: `{:?} -> {}`", cards, json);

    fs::write(file_path, json)?;

    Ok(())
}
