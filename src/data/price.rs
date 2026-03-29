use std::{fs, path::Path};

use anyhow::Result;
use serde_derive::Deserialize;

const PRICES_CONFIG: &str = "config/prices.toml";

#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    #[serde(default)]
    pub money: u16,
    #[serde(default)]
    pub wood: u8,
    #[serde(default)]
    pub food: u8,
    #[serde(default)]
    pub metal: u8,
    #[serde(default)]
    pub weapon: u8,
    #[serde(default)]
    pub wax: u8,
    #[serde(default)]
    pub wool: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Reward {
    #[serde(default)]
    pub money: u16,
    #[serde(default)]
    pub wood: u8,
    #[serde(default)]
    pub food: u8,
    #[serde(default)]
    pub metal: u8,
    #[serde(default)]
    pub weapon: u8,
    #[serde(default)]
    pub wax: u8,
    #[serde(default)]
    pub wool: u8,
    #[serde(default)]
    pub reputation: u8,
    #[serde(default)]
    pub people: u8,
    #[serde(default)]
    pub law_card: u8,
    #[serde(default)]
    pub action_card: u8,
    #[serde(default)]
    pub war_card: u8,
}

/// Диапазон цен для ресурса на рынке (min — D6=1, max — D6=6)
#[derive(Debug, Clone, Deserialize)]
pub struct ResourcePriceRange {
    pub min: u16,
    pub max: u16,
}

impl ResourcePriceRange {
    /// Средняя цена диапазона (округление вниз)
    pub fn average(&self) -> u16 {
        (self.min + self.max) / 2
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorePrices {
    pub workshop_feeds: Price,
    pub workshop_wools: Price,
    pub workshop_waxes: Price,
    pub workshop_weapons: Price,
    pub workshop_church: Price,
    pub workshop_school: Price,
    pub card_war: Price,
    pub card_law: Price,
    pub card_action: Price,
    pub field_food: Price,
    pub field_wood: Price,
    pub resource_wood: Price,
    pub resource_food: Price,
    pub resource_metal: Price,
    pub resource_weapon: Price,
    pub resource_wax: Price,
    pub resource_wool: Price,
    pub range_wood: ResourcePriceRange,
    pub range_food: ResourcePriceRange,
    pub range_metal: ResourcePriceRange,
    pub range_weapon: ResourcePriceRange,
    pub range_wax: ResourcePriceRange,
    pub range_wool: ResourcePriceRange,
}

impl Price {
    /// Проверяет, может ли игрок заплатить эту цену
    pub fn can_afford(&self, player: &crate::game::player::Player) -> bool {
        player.money >= self.money
            && player.wood >= self.wood
            && player.food >= self.food
            && player.metal >= self.metal
            && player.weapon >= self.weapon
            && player.wax >= self.wax
            && player.wool >= self.wool
    }

    /// Записывает потребности по этой цене (вызывается при нехватке ресурсов).
    /// Использует max-семантику put_*: запоминает наибольшую потребность.
    pub fn record_needs(&self, needs: &mut crate::data::needs::Needs) {
        if self.money > 0 {
            needs.put_money(self.money.min(255) as u8);
        }
        if self.wood > 0 {
            needs.put_wood(self.wood);
        }
        if self.food > 0 {
            needs.put_food(self.food);
        }
        if self.metal > 0 {
            needs.put_metal(self.metal);
        }
        if self.weapon > 0 {
            needs.put_weapon(self.weapon);
        }
        if self.wax > 0 {
            needs.put_wax(self.wax);
        }
        if self.wool > 0 {
            needs.put_wool(self.wool);
        }
    }

    /// Списывает цену с игрока через Game (ресурсы возвращаются в хранилище)
    pub fn deduct(&self, game: &mut crate::game::Game, player_hash: u64) {
        use crate::data::resource::CubeResourceTypeEnum;
        if self.money > 0 {
            game.take_money(player_hash, self.money);
        }
        if self.wood > 0 {
            game.take_cube_resource(player_hash, CubeResourceTypeEnum::Wood, self.wood);
        }
        if self.food > 0 {
            game.take_cube_resource(player_hash, CubeResourceTypeEnum::Food, self.food);
        }
        if self.metal > 0 {
            game.take_cube_resource(player_hash, CubeResourceTypeEnum::Metal, self.metal);
        }
        if self.weapon > 0 {
            game.take_cube_resource(player_hash, CubeResourceTypeEnum::Weapon, self.weapon);
        }
        if self.wax > 0 {
            game.take_cube_resource(player_hash, CubeResourceTypeEnum::Wax, self.wax);
        }
        if self.wool > 0 {
            game.take_cube_resource(player_hash, CubeResourceTypeEnum::Wool, self.wool);
        }
    }
}

impl StorePrices {
    fn load_prices_config() -> Result<StorePrices> {
        let toml_content = fs::read_to_string(Path::new(PRICES_CONFIG))?;
        let prices: StorePrices = toml::from_str(&toml_content)?;
        Ok(prices)
    }

    pub fn new() -> Result<StorePrices> {
        Ok(Self::load_prices_config()?)
    }

    /// Возвращает цену мастерской по типу
    pub fn get_workshop_price(
        &self,
        workshop_type: &crate::data::workshop::WorkshopTypeEnum,
    ) -> &Price {
        use crate::data::workshop::WorkshopTypeEnum;
        match workshop_type {
            WorkshopTypeEnum::Weapons => &self.workshop_weapons,
            WorkshopTypeEnum::Feeds => &self.workshop_feeds,
            WorkshopTypeEnum::Waxes => &self.workshop_waxes,
            WorkshopTypeEnum::Wools => &self.workshop_wools,
            WorkshopTypeEnum::Church => &self.workshop_church,
            WorkshopTypeEnum::School => &self.workshop_school,
        }
    }

    /// Возвращает цену земли по типу
    pub fn get_field_price(
        &self,
        field_type: &crate::game::fields_desk::FieldPlaceTypeEnum,
    ) -> &Price {
        use crate::game::fields_desk::FieldPlaceTypeEnum;
        match field_type {
            FieldPlaceTypeEnum::Food => &self.field_food,
            FieldPlaceTypeEnum::Wood => &self.field_wood,
        }
    }

    /// Возвращает цену ресурса по типу
    pub fn get_resource_price(
        &self,
        resource_type: &crate::data::resource::CubeResourceTypeEnum,
    ) -> &Price {
        use crate::data::resource::CubeResourceTypeEnum;
        match resource_type {
            CubeResourceTypeEnum::Wood => &self.resource_wood,
            CubeResourceTypeEnum::Food => &self.resource_food,
            CubeResourceTypeEnum::Metal => &self.resource_metal,
            CubeResourceTypeEnum::Weapon => &self.resource_weapon,
            CubeResourceTypeEnum::Wax => &self.resource_wax,
            CubeResourceTypeEnum::Wool => &self.resource_wool,
        }
    }

    /// Возвращает диапазон цен для ресурса на рынке
    pub fn get_resource_range(
        &self,
        resource_type: &crate::data::resource::CubeResourceTypeEnum,
    ) -> &ResourcePriceRange {
        use crate::data::resource::CubeResourceTypeEnum;
        match resource_type {
            CubeResourceTypeEnum::Wood => &self.range_wood,
            CubeResourceTypeEnum::Food => &self.range_food,
            CubeResourceTypeEnum::Metal => &self.range_metal,
            CubeResourceTypeEnum::Weapon => &self.range_weapon,
            CubeResourceTypeEnum::Wax => &self.range_wax,
            CubeResourceTypeEnum::Wool => &self.range_wool,
        }
    }
}
