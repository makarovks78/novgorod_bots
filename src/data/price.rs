use std::{fs, path::Path};

use serde_derive::Deserialize;
use anyhow::Result;

const PRICES_CONFIG: &str = "config/prices.toml";

#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    #[serde(default)] pub money: u16,
    #[serde(default)] pub wood: u8,
    #[serde(default)] pub food: u8,
    #[serde(default)] pub metal: u8,
    #[serde(default)] pub weapon: u8,
    #[serde(default)] pub wax: u8,
    #[serde(default)] pub wool: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Reward {
    #[serde(default)] pub money: u16,
    #[serde(default)] pub wood: u8,
    #[serde(default)] pub food: u8,
    #[serde(default)] pub metal: u8,
    #[serde(default)] pub weapon: u8,
    #[serde(default)] pub wax: u8,
    #[serde(default)] pub wool: u8,
    #[serde(default)] pub reputation: u8,
    #[serde(default)] pub people: u8,
    #[serde(default)] pub law_card: u8,
    #[serde(default)] pub action_card: u8,
    #[serde(default)] pub war_card: u8,
}


#[derive(Debug, Deserialize)]
pub struct StorePrices {
    pub workshop_feeds: Price,
    pub workshop_wools: Price,
    pub workshop_waxes: Price,
    pub workshop_weapons: Price,
    pub workshop_church: Price,
    pub workshop_school: Price,
    pub card_war: Price,
    pub card_law: Price,
    pub card_action: Price,
}

impl StorePrices {
    fn load_prices_config() -> Result<StorePrices> {
        let toml_content = fs::read_to_string(Path::new(PRICES_CONFIG))?;
        let prices: StorePrices = toml::from_str(&toml_content)?;
        Ok(prices)
    }
}
