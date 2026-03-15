use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WarCard {
    level: u8,
    attack: u8,
    defense: u8,
    defense_in_attack: u8,
    attacks_in_defense: u8,
}

#[derive(Debug, Deserialize)]
pub struct ActionCard {}

#[derive(Debug, Deserialize)]
pub struct LawCard {}
