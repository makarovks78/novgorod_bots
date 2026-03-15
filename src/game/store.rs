use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;

const STORE_CONFIG: &str = "config/store.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Store {
    pub wood: u8,
    pub food: u8,
    pub metal: u8,
    pub weapon: u8,
    pub wax: u8,
    pub wool: u8,
    pub money: u16,
    pub people: u8,
    pub weapons: u8,
    pub feeds: u8,
    pub waxes: u8,
    pub wools: u8,
    pub church: u8,
    pub school: u8,
    pub action_cards: Vec<u64>,
    pub law_cards: Vec<u64>,
    pub war_cards_infantry: Vec<u64>,
    pub war_cards_cavalry: Vec<u64>,
    pub war_cards_archer: Vec<u64>,
}

impl Store {
    pub fn new() -> Result<Self> {
        let store = Self::load_store_stuff()?;
        Ok(store)
    }

    pub fn to_string(&self) -> String {
        format!(
            "│ ├ Ресурсы: Дерево: {wood}, Еда: {food}, Металл: {metal}, Оружие: {weapon}, Воск: {wax}, Пушнина: {wool}, Монеты: {money}, Миплы: {people}\n\
            │ ├ Мастерские: Кузня {weapons}, Едальня {feeds}, Восковая {waxes}, Кожевенная {wools}, Церковь {church}, Школа {school}\n\
            │ └ Карты: События: {actions}, Закон: {law}, Война: пехота {war_infantry} кавалерия {war_cavalry} лучники {war_archers}",
            wood = self.wood,
            food = self.food,
            metal = self.metal,
            weapon = self.weapon,
            wax = self.wax,
            wool = self.wool,
            money = self.money,
            people = self.people,
            weapons = self.weapons,
            feeds = self.feeds,
            waxes = self.waxes,
            wools = self.wools,
            church = self.church,
            school = self.school,
            actions = self.action_cards.len(),
            law = self.law_cards.len(),
            war_infantry = self.war_cards_infantry.len(),
            war_cavalry = self.war_cards_cavalry.len(),
            war_archers = self.war_cards_archer.len(),
        )
    }

    fn load_store_stuff() -> Result<Store> {
        let toml_content = fs::read_to_string(Path::new(STORE_CONFIG))?;
        let stuff: Store = toml::from_str(&toml_content)?;
        Ok(stuff)
    }

}
