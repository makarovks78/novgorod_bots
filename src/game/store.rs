use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;

use crate::data::workshop::WorkshopTypeEnum;
use crate::game::player::MarketStall;

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
    /// Настроение народа (1-6), влияет на порог голосования
    #[serde(default = "default_mood")]
    pub mood: u8,
    /// Рыночные лотки магазина (НЕ сериализуются, создаются в рантайме)
    #[serde(skip)]
    pub stall_wood: MarketStall,
    #[serde(skip)]
    pub stall_food: MarketStall,
    #[serde(skip)]
    pub stall_metal: MarketStall,
    #[serde(skip)]
    pub stall_weapon: MarketStall,
    #[serde(skip)]
    pub stall_wax: MarketStall,
    #[serde(skip)]
    pub stall_wool: MarketStall,
}

fn default_mood() -> u8 {
    3
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

    /// Проверяет, есть ли мастерская данного типа в запасе
    pub fn has_workshop(&self, workshop_type: &WorkshopTypeEnum) -> bool {
        match workshop_type {
            WorkshopTypeEnum::Weapons => self.weapons > 0,
            WorkshopTypeEnum::Feeds => self.feeds > 0,
            WorkshopTypeEnum::Waxes => self.waxes > 0,
            WorkshopTypeEnum::Wools => self.wools > 0,
            WorkshopTypeEnum::Church => self.church > 0,
            WorkshopTypeEnum::School => self.school > 0,
        }
    }

    /// Уменьшает запас мастерских данного типа на 1
    pub fn take_workshop(&mut self, workshop_type: &WorkshopTypeEnum) {
        match workshop_type {
            WorkshopTypeEnum::Weapons => self.weapons = self.weapons.saturating_sub(1),
            WorkshopTypeEnum::Feeds => self.feeds = self.feeds.saturating_sub(1),
            WorkshopTypeEnum::Waxes => self.waxes = self.waxes.saturating_sub(1),
            WorkshopTypeEnum::Wools => self.wools = self.wools.saturating_sub(1),
            WorkshopTypeEnum::Church => self.church = self.church.saturating_sub(1),
            WorkshopTypeEnum::School => self.school = self.school.saturating_sub(1),
        }
    }

    /// Проверяет, есть ли ресурс данного типа в хранилище
    pub fn has_resource(&self, resource: &crate::data::resource::CubeResourceTypeEnum) -> bool {
        use crate::data::resource::CubeResourceTypeEnum;
        match resource {
            CubeResourceTypeEnum::Wood => self.wood > 0,
            CubeResourceTypeEnum::Food => self.food > 0,
            CubeResourceTypeEnum::Metal => self.metal > 0,
            CubeResourceTypeEnum::Weapon => self.weapon > 0,
            CubeResourceTypeEnum::Wax => self.wax > 0,
            CubeResourceTypeEnum::Wool => self.wool > 0,
        }
    }

    /// Возвращает ссылку на рыночный лоток магазина по типу ресурса
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

    /// Возвращает мутабельную ссылку на рыночный лоток магазина
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
}
