use crate::game::main_desk::EnemyRegionEnum;
use anyhow::Result;
use rand::{rng, seq::SliceRandom};
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

    /// Возвращает индекс случайного свободного поля данного типа, или None
    pub fn get_free_field(&self, field_type: &FieldPlaceTypeEnum) -> Option<usize> {
        let mut rng = rng();
        let mut candidates: Vec<usize> = self
            .places
            .iter()
            .enumerate()
            .filter(|(_, place)| {
                place.player_hash.is_none()
                    && Self::field_type_matches(&place.field_type, field_type)
            })
            .map(|(idx, _)| idx)
            .collect();
        candidates.shuffle(&mut rng);
        candidates.first().copied()
    }

    /// Покупает поле — устанавливает владельца
    pub fn buy_field(&mut self, field_index: usize, player_hash: u64) {
        self.places[field_index].player_hash = Some(player_hash);
    }

    /// Возвращает VP за поле
    pub fn get_field_vp(&self, field_index: usize) -> u8 {
        self.places[field_index].vp
    }

    /// Возвращает все поля игрока данного типа (индекс + количество ресурсов)
    pub fn get_player_fields_by_type(
        &self,
        player_hash: u64,
        field_type: &FieldPlaceTypeEnum,
    ) -> Vec<(usize, u8)> {
        self.places
            .iter()
            .enumerate()
            .filter(|(_, place)| {
                place.player_hash == Some(player_hash)
                    && Self::field_type_matches(&place.field_type, field_type)
            })
            .map(|(idx, place)| (idx, place.qty))
            .collect()
    }

    fn field_type_matches(a: &FieldPlaceTypeEnum, b: &FieldPlaceTypeEnum) -> bool {
        matches!(
            (a, b),
            (FieldPlaceTypeEnum::Wood, FieldPlaceTypeEnum::Wood)
                | (FieldPlaceTypeEnum::Food, FieldPlaceTypeEnum::Food)
        )
    }

    /// Считает количество полей игрока с фигуркой
    pub fn count_player_miples(&self, player_hash: u64) -> u8 {
        self.places
            .iter()
            .filter(|p| p.player_hash == Some(player_hash) && p.is_miple)
            .count() as u8
    }

    /// Возвращает индексы полей игрока без фигурки
    pub fn get_player_chip_fields(&self, player_hash: u64) -> Vec<usize> {
        self.places
            .iter()
            .enumerate()
            .filter(|(_, p)| p.player_hash == Some(player_hash) && !p.is_miple)
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Ставит фигурку на поле
    pub fn set_miple(&mut self, field_index: usize) {
        self.places[field_index].is_miple = true;
    }

    /// Убирает фигурку с поля (ставит фишку)
    pub fn remove_miple(&mut self, field_index: usize) {
        self.places[field_index].is_miple = false;
    }

    /// Возвращает все поля игрока данного типа с фигуркой (только работающие)
    pub fn get_player_active_fields_by_type(
        &self,
        player_hash: u64,
        field_type: &FieldPlaceTypeEnum,
    ) -> Vec<(usize, u8)> {
        self.places
            .iter()
            .enumerate()
            .filter(|(_, place)| {
                place.player_hash == Some(player_hash)
                    && place.is_miple
                    && Self::field_type_matches(&place.field_type, field_type)
            })
            .map(|(idx, place)| (idx, place.qty))
            .collect()
    }
}
