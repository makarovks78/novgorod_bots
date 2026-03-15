use anyhow::Result;
use rand::{rng, seq::SliceRandom};
use serde_derive::Deserialize;
use std::fmt::Debug;

use crate::data::workshop::{InputResources, WorkshopTypeEnum};

const MAIN_DESK_CONFIG: &str = "config/main_desk.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct MainDesk {
    places: Vec<WorkshopPlace>,
    commander: u64,
    archbishop: u64,
    mayor: u64,
    enemy_polock: u8,
    enemy_sweden: u8,
    enemy_vladimir: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub enum RegionEnum {
    Ludin,
    Plotnetsky,
    Slavensky,
    Nereevsky,
    Zagorodsky,
}

#[derive(Debug, Deserialize, Clone)]
pub enum EnemyRegionEnum {
    Vladimir,
    Polock,
    Sweden,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorkshopPlace {
    pub id: u8,
    pub nhs: Vec<u8>,
    pub region: RegionEnum,
    #[serde(default)]
    pub workshop_type: Option<WorkshopTypeEnum>,
    #[serde(default)]
    pub input_resources: Vec<InputResources>,
    #[serde(default)]
    pub current_input_resource: InputResources,
    #[serde(default)]
    pub is_miple: bool,
    #[serde(default)]
    pub owner_hash: Option<u64>,
    #[serde(default)]
    pub master: Option<u8>,
}

impl MainDesk {
    fn load_main_desk_config() -> Result<MainDesk> {
        let content = std::fs::read_to_string(MAIN_DESK_CONFIG)?;
        let main_desk: MainDesk = toml::from_str(&content)?;
        Ok(main_desk)
    }

    pub fn new() -> Result<MainDesk> {
        Ok(Self::load_main_desk_config()?)
    }

    pub fn is_player_archbishop(&self, player_hash: u64) -> bool {
        player_hash == self.archbishop
    }

    pub fn is_player_commander(&self, player_hash: u64) -> bool {
        player_hash == self.commander
    }

    pub fn is_player_mayor(&self, player_hash: u64) -> bool {
        player_hash == self.mayor
    }

    pub fn get_free_place(&self) -> Option<u8> {
        let mut rng = rng();
        let mut shuffled_places: Vec<_> = self.places.iter().collect();
        shuffled_places.shuffle(&mut rng);

        shuffled_places
            .iter()
            .find(|place| place.owner_hash.is_none())
            .map(|place| place.id)
    }

    pub fn get_free_place_with_free_nhs(&self) -> Option<u8> {
        let mut rng = rng();
        let mut shuffled_places: Vec<_> = self.places.iter().collect();
        shuffled_places.shuffle(&mut rng);

        shuffled_places
            .iter()
            .find(|place| {
                // Check if this place is free
                if place.owner_hash.is_some() {
                    return false;
                }

                // Check if any of the neighboring places are free
                place.nhs.iter().any(|&neighbor_id| {
                    self.places
                        .iter()
                        .find(|p| p.id == neighbor_id)
                        .map_or(false, |neighbor| neighbor.owner_hash.is_none())
                })
            })
            .map(|place| place.id)
    }

    pub fn set_workshop_and_owner(
        &mut self,
        place_id: u8,
        player_hash: u64,
        workshop_type: WorkshopTypeEnum,
    ) -> Result<(), String> {
        let place = self
            .places
            .iter_mut()
            .find(|p| p.id == place_id)
            .ok_or_else(|| format!("Place with id {} not found", place_id))?;

        if place.owner_hash.is_some() {
            return Err(format!("Place {} is already occupied", place_id));
        }

        place.owner_hash = Some(player_hash);
        place.workshop_type = Some(workshop_type);

        Ok(())
    }

    pub fn count_player_places_by_type(
        &self,
        player_hash: u64,
        workshop_type: WorkshopTypeEnum,
    ) -> i16 {
        self.places
            .iter()
            .filter(|place| {
                place.owner_hash == Some(player_hash)
                    && place.workshop_type.as_ref() == Some(&workshop_type)
            })
            .count() as i16
    }

    pub fn get_player_places(&self, player_hash: u64) -> Vec<&WorkshopPlace> {
        self.places
            .iter()
            .filter(|place| place.owner_hash == Some(player_hash))
            .collect()
    }

    pub fn count_player_workshops(
        &self,
        player_hash: u64,
        workshop_type: WorkshopTypeEnum,
    ) -> usize {
        self.places
            .iter()
            .filter(|place| {
                place.owner_hash == Some(player_hash)
                    && place.workshop_type.as_ref() == Some(&workshop_type)
            })
            .count()
    }
}
