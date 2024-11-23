use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::cli::LanguageCode;

#[derive(Debug, Deserialize, Serialize)]
pub struct Localizer {
    pub hostname: String,

    pub colors: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
    pub categories: HashMap<String, String>,
    pub rarities: HashMap<String, String>,
}

impl Localizer {
    fn reverse_search(hash_map: &HashMap<String, String>, value: &str) -> Option<String> {
        hash_map.iter().find_map(|(key, val)| {
            if val == value {
                Some(key.to_string())
            } else {
                None
            }
        })
    }

    pub fn match_color(&self, value: &str) -> Option<String> {
        Self::reverse_search(&self.colors, value)
    }

    pub fn match_attribute(&self, value: &str) -> Option<String> {
        Self::reverse_search(&self.attributes, value)
    }

    pub fn match_category(&self, value: &str) -> Option<String> {
        Self::reverse_search(&self.categories, value)
    }

    pub fn match_rarity(&self, value: &str) -> Option<String> {
        Self::reverse_search(&self.rarities, value)
    }

    pub fn load(language: LanguageCode) -> Result<Localizer> {
        match language {
            LanguageCode::ChineseHongKong => Self::load_from_file("zh_hk"),
            LanguageCode::ChineseSimplified => Self::load_from_file("zh_cn"),
            LanguageCode::ChineseTaiwan => Self::load_from_file("zh_tw"),
            LanguageCode::English => Self::load_from_file("en"),
            LanguageCode::EnglishAsia => Self::load_from_file("en_asia"),
            LanguageCode::Japanese => Self::load_from_file("jp"),
            LanguageCode::Thai => Self::load_from_file("th"),
        }
    }

    pub fn load_from_file(locale: &str) -> Result<Localizer> {
        let path = format!("./locales/{}.toml", locale);
        let path = PathBuf::from(path);
        info!("load {} locale from: {}", locale, path.to_string_lossy());

        let locale_data = fs::read_to_string(&path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        debug!("loaded {}", locale_data);

        let localizer: Localizer = toml::from_str(&locale_data)?;
        Ok(localizer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(String::from("foo"), String::from("Toto"));
        map.insert(String::from("bar"), String::from("Tata"));
        map.insert(String::from("baz"), String::from("Tutu"));

        map
    }

    #[test]
    fn reverse_search_returns_some() {
        let map = get_test_map();

        let actual = Localizer::reverse_search(&map, "Toto");
        let expected = Some(String::from("foo"));

        assert_eq!(actual, expected);
    }

    #[test]
    fn reverse_search_returns_none() {
        let map = get_test_map();

        let actual = Localizer::reverse_search(&map, "Titi");
        let expected = None;

        assert_eq!(actual, expected);
    }
}
