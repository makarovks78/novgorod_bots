use anyhow::Result;
use serde_derive::Deserialize;

use crate::{data::needs::Needs, utils::generate_hash};

const PLAYER_DEFAULTS_CONFIG: &str = "config/player_defaults.toml";

#[derive(Deserialize)]
pub struct PlayerDefaults {
    pub wood: u8,
    pub food: u8,
    pub metal: u8,
    pub weapon: u8,
    pub wax: u8,
    pub wool: u8,
    pub money: u16,
    pub people: u8,
    pub reputation: u8,
}

impl PlayerDefaults {
    pub fn load_defaults() -> Result<PlayerDefaults> {
        let content = std::fs::read_to_string(PLAYER_DEFAULTS_CONFIG)?;
        let player_defaults: PlayerDefaults = toml::from_str(&content)?;
        Ok(player_defaults)
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub hash: u64,
    pub name: String,
    pub is_first: bool,
    pub wood: u8,
    pub food: u8,
    pub metal: u8,
    pub weapon: u8,
    pub wax: u8,
    pub wool: u8,
    pub money: u16,
    pub people: u8,
    pub vp: u8,
    pub reputation: u8,
    pub action_cards: Vec<u64>,
    pub law_cards: Vec<u64>,
    pub war_cards_infantry: Vec<u64>,
    pub war_cards_cavalry: Vec<u64>,
    pub war_cards_archer: Vec<u64>,
    pub needs: Needs
}

impl Player {
    pub fn new(name: String) -> Player {
        let hash = generate_hash();
        Player {
            hash,
            name,
            wood: 0,
            food: 0,
            metal: 0,
            weapon: 0,
            wax: 0,
            wool: 0,
            money: 0,
            people: 0,
            vp: 0,
            reputation: 0,
            action_cards: vec![],
            law_cards: vec![],
            war_cards_infantry: vec![],
            war_cards_cavalry: vec![],
            war_cards_archer: vec![],
            is_first: false,
            needs: Needs::new()
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "│ ├ Игрок: {name}, Победные очки: {vp}, Репутация: {reputation}\n\
            │ ├ Ресурсы: Дерево: {wood}, Еда: {food}, Металл: {metal}, Оружие: {weapon}, Воск: {wax}, Пушнина: {wool}, Монеты: {money}, Миплы: {people}\n\
            │ ├ Карты: Закон: {law}, Действия: {action}, Война: пехота {war_infantry} кавалерия {war_cavalry} лучники {war_archers}\n\
            │ └ Нужды:  Дерево: {nwood}, Еда: {nfood}, Металл: {nmetal}, Оружие: {nweapon}, Воск: {nwax}, Пушнина: {nwool}, Монеты: {nmoney}, Миплы: {npeople}\n
            │           Репутация: {nreputation}, Закон: {nlaw}, Действия: {naction}, Война: {nwar}\n" ,
            name = self.name,
            wood = self.wood,
            food = self.food,
            metal = self.metal,
            weapon = self.weapon,
            wax = self.wax,
            wool = self.wool,
            money = self.money,
            people = self.people,
            vp = self.vp,
            reputation = self.reputation,
            law = self.law_cards.len(),
            action = self.action_cards.len(),
            war_infantry = self.war_cards_infantry.len(),
            war_cavalry = self.war_cards_cavalry.len(),
            war_archers = self.war_cards_archer.len(),
            nwood = self.needs.last.wood,
            nfood = self.needs.last.food,
            nmetal = self.needs.last.metal,
            nweapon = self.needs.last.weapon,
            nwax = self.needs.last.wax,
            nwool = self.needs.last.wool,
            nmoney = self.needs.last.money,
            npeople = self.needs.last.people,
            nreputation = self.needs.last.reputation,
            nlaw = self.needs.last.law_cards,
            naction = self.needs.last.action_cards,
            nwar = self.needs.last.war_cards
        )
    }
}
