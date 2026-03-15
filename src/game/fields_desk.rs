use crate::game::main_desk::EnemyRegionEnum;
use anyhow::Result;
use serde_derive::Deserialize;

const FIELD_DESK_CONFIG: &str = "config/field_desk.toml";

#[derive(Debug, Deserialize, Clone)]
pub enum FieldPlaceTypeEnum {
    Wood,
    Food,
}

impl FieldPlaceTypeEnum {
    pub fn name(&self) -> &str {
        match self {
            FieldPlaceTypeEnum::Wood => "Дерево",
            FieldPlaceTypeEnum::Food => "Еда",
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FieldPlace {
    pub nhs: Vec<u8>,
    pub vp: u8,
    pub start: Option<u8>,
    pub field_type: FieldPlaceTypeEnum,
    pub enemy: EnemyRegionEnum,
    pub qty: u8,
    #[serde(default)]
    pub is_miple: bool,
    pub player_hash: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FieldDesk {
    pub places: Vec<FieldPlace>,
}

impl FieldDesk {
    fn load_field_desk_config() -> Result<FieldDesk> {
        let content = std::fs::read_to_string(FIELD_DESK_CONFIG)?;
        let field_desk: FieldDesk = toml::from_str(&content)?;
        Ok(field_desk)
    }

    pub fn new() -> Result<FieldDesk> {
        Ok(Self::load_field_desk_config()?)
    }
}
