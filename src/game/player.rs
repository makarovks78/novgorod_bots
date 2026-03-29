use anyhow::Result;
use serde_derive::Deserialize;

use crate::{data::needs::Needs, utils::generate_hash};

const PLAYER_DEFAULTS_CONFIG: &str = "config/player_defaults.toml";

/// Рыночный лоток: ресурс, количество, цена за единицу
#[derive(Debug, Clone, Default)]
pub struct MarketStall {
    pub qty: u8,
    pub price: u16,
}

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
    pub needs: Needs,
    /// Рыночные лотки (по одному на каждый тип ресурса)
    pub stall_wood: MarketStall,
    pub stall_food: MarketStall,
    pub stall_metal: MarketStall,
    pub stall_weapon: MarketStall,
    pub stall_wax: MarketStall,
    pub stall_wool: MarketStall,
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
            needs: Needs::new(),
            stall_wood: MarketStall::default(),
            stall_food: MarketStall::default(),
            stall_metal: MarketStall::default(),
            stall_weapon: MarketStall::default(),
            stall_wax: MarketStall::default(),
            stall_wool: MarketStall::default(),
        }
    }

    /// Возвращает ссылку на рыночный лоток по типу ресурса
    pub fn get_stall(
        &self,
        resource: &crate::data::resource::CubeResourceTypeEnum,
    ) -> &MarketStall {
        use crate::data::resource::CubeResourceTypeEnum;
        match resource {
            CubeResourceTypeEnum::Wood => &self.stall_wood,
            CubeResourceTypeEnum::Food => &self.stall_food,
            CubeResourceTypeEnum::Metal => &self.stall_metal,
            CubeResourceTypeEnum::Weapon => &self.stall_weapon,
            CubeResourceTypeEnum::Wax => &self.stall_wax,
            CubeResourceTypeEnum::Wool => &self.stall_wool,
        }
    }

    /// Возвращает мутабельную ссылку на рыночный лоток по типу ресурса
    pub fn get_stall_mut(
        &mut self,
        resource: &crate::data::resource::CubeResourceTypeEnum,
    ) -> &mut MarketStall {
        use crate::data::resource::CubeResourceTypeEnum;
        match resource {
            CubeResourceTypeEnum::Wood => &mut self.stall_wood,
            CubeResourceTypeEnum::Food => &mut self.stall_food,
            CubeResourceTypeEnum::Metal => &mut self.stall_metal,
            CubeResourceTypeEnum::Weapon => &mut self.stall_weapon,
            CubeResourceTypeEnum::Wax => &mut self.stall_wax,
            CubeResourceTypeEnum::Wool => &mut self.stall_wool,
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
