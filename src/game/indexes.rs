use anyhow::Result;
use serde_derive::Deserialize;
use std::collections::HashMap;

const INDEX_CONFIG: &str = "config/indexes.toml";
const MAX_INDEX: u8 = 20;

#[derive(Deserialize, Clone)]
pub struct Indexes {
    #[serde(deserialize_with = "crate::deserializer::deserialize_u8_key_map")]
    pub people_food_map: HashMap<u8, u8>,
    #[serde(deserialize_with = "crate::deserializer::deserialize_u8_key_map")]
    pub people_resource_map: HashMap<u8, u8>,
    #[serde(deserialize_with = "crate::deserializer::deserialize_u8_key_map")]
    pub people_workshop_map: HashMap<u8, u8>,
    #[serde(deserialize_with = "crate::deserializer::deserialize_u8_key_map")]
    pub people_vp_map: HashMap<u8, u8>,
    #[serde(deserialize_with = "crate::deserializer::deserialize_u8_key_map")]
    pub reputation_people_map: HashMap<u8, u8>,
    #[serde(deserialize_with = "crate::deserializer::deserialize_u8_key_map")]
    pub reputation_vp_map: HashMap<u8, u8>,
}

impl Indexes {
    fn load_indexes_config() -> Result<Indexes> {
        let content = std::fs::read_to_string(INDEX_CONFIG)?;
        let indexes: Indexes = toml::from_str(&content)?;
        Ok(indexes)
    }

    pub fn new() -> Result<Self> {
        Ok(Self::load_indexes_config()?)
    }
}
