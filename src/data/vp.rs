use std::{fs, path::Path};

use anyhow::Result;
use serde_derive::Deserialize;

use crate::game::{fields_desk::FieldDesk, indexes::Indexes, main_desk::MainDesk, player::Player};

const VP_CONFIG: &str = "config/vp.toml";

#[derive(Deserialize)]
pub struct VictoryPointsConfig {
    pub weapons: f32,
    pub feeds: f32,
    pub waxes: f32,
    pub wools: f32,
    pub church: f32,
    pub school: f32,
    pub archbishop: f32,
    pub mayor: f32,
    pub commander: f32,
    pub wood_field: f32,
    pub eat_field: f32,
    pub wood: f32,
    pub food: f32,
    pub metal: f32,
    pub weapon: f32,
    pub wax: f32,
    pub wool: f32,
    pub money: f32,
    pub people: f32,
    pub vp: f32,
    pub reputation: f32,
    pub action_cards: f32,
    pub law_cards: f32,
    pub war_cards_infantry: f32,
    pub war_cards_cavalry: f32,
    pub war_cards_archer: f32,
}

impl VictoryPointsConfig {
    pub fn load_from_config() -> Result<Self> {
        let toml_content = fs::read_to_string(Path::new(VP_CONFIG))?;
        let vp: VictoryPointsConfig = toml::from_str(&toml_content)?;
        Ok(vp)
    }

    pub fn calculate_vp(
        &self,
        player: &Player,
        main_desk: &MainDesk,
        field_desk: &FieldDesk,
        indexes: &Indexes,
    ) -> i16 {
        let vp: f32 = self.weapon * player.weapon as f32
            + self.food * player.food as f32
            + self.metal * player.metal as f32
            + self.wood * player.wood as f32
            + self.wax * player.wax as f32
            + self.wool * player.wool as f32
            + self.weapon * player.weapon as f32
            + self.people * player.people as f32
            + self.vp * player.vp as f32
            + self.reputation * player.reputation as f32
            + self.money * player.money as f32
            + self.action_cards * player.action_cards.len() as f32
            + self.law_cards * player.law_cards.len() as f32
            + self.war_cards_infantry * player.war_cards_infantry.len() as f32
            + self.war_cards_cavalry * player.war_cards_cavalry.len() as f32
            + self.war_cards_archer * player.war_cards_archer.len() as f32
            + indexes
                .people_vp_map
                .get(&player.people)
                .unwrap()
                .clone() as f32
            + indexes
                .reputation_vp_map
                .get(&player.reputation)
                .unwrap()
                .clone() as f32;
        vp.floor() as i16
    }
}
