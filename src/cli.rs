use std::{ffi::OsString, path::PathBuf, str::FromStr};

use anyhow::Result;
use clap::{command, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "veganet")]
#[command(
    about = "Dynamically fetch data for the One Piece TCG from official sites.",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Language to use for the data
    #[arg(short, long, alias = "lang", value_name = "LANGUAGE", default_value_t = LanguageCode::English, value_enum)]
    pub language: LanguageCode,

    // /// Write data to a file instead of stdout
    // #[arg(short, long = "output", value_name = "file")]
    // pub output_file: PathBuf,
    //
    /// Outputs information in JSON
    // #[arg(short = 'j', long = "json")]
    // pub output_json: bool,

    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Get the list of all existing packs
    #[command(alias = "pack", alias = "pak")]
    Packs,
    /// Get all cards within the given pack
    #[command(alias = "card", alias = "car")]
    Cards {
        /// ID of the pack
        pack_id: OsString,
    },
    /// Download all card images for a given pack
    #[command(alias = "image", alias = "img")]
    Images {
        /// ID of the pack
        pack_id: OsString,
        /// Directory where the images should be saved
        #[arg(short, long = "output-dir")]
        output_dir: PathBuf,
    },
    /// Launch into interactive mode
    #[command(name = "inter", alias = "interactive", alias = "int")]
    Interactive,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LanguageCode {
    #[value(name = "chinese-hongkong", alias = "zh_hk", alias = "zh_HK")]
    ChineseHongKong,
    #[value(name = "chinese-simplified", alias = "zh_cn", alias = "zh_CN")]
    ChineseSimplified,
    #[value(name = "chinese-taiwan", alias = "zh_tw", alias = "zh_TW")]
    ChineseTaiwan,
    #[value(name = "english", alias = "en")]
    English,
    #[value(name = "english-asia", alias = "en-asia")]
    EnglishAsia,
    #[value(name = "japanese", alias = "jp")]
    Japanese,
    #[value(name = "thai", alias = "th")]
    Thai,
}

impl LanguageCode {
    pub fn to_path(self) -> PathBuf {
        let path = match self {
            LanguageCode::ChineseHongKong => "chinese-hong-kong",
            LanguageCode::ChineseSimplified => "chinese-simplified",
            LanguageCode::ChineseTaiwan => "chinese-taiwan",
            LanguageCode::English => "english",
            LanguageCode::EnglishAsia => "english-asia",
            LanguageCode::Japanese => "japanese",
            LanguageCode::Thai => "thai",
        };

        PathBuf::from(path)
    }
}

impl FromStr for LanguageCode {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "chinese-hongkong" => Ok(LanguageCode::ChineseHongKong),
            "chinese-simplified" => Ok(LanguageCode::ChineseSimplified),
            "chinese-taiwan" => Ok(LanguageCode::ChineseTaiwan),
            "english" => Ok(LanguageCode::English),
            "english-asia" => Ok(LanguageCode::EnglishAsia),
            "japanese" => Ok(LanguageCode::Japanese),
            "thai" => Ok(LanguageCode::Thai),
            _ => Err(()),
        }
    }
}
