use crate::game_info;
use log::info;

use crate::data::price::StorePrices;
use crate::data::resource::CubeResourceTypeEnum;
use crate::data::vp::VictoryPointsConfig;
use crate::game::fields_desk::FieldDesk;
use crate::game::indexes::Indexes;
use crate::game::main_desk::MainDesk;
use crate::game::player::{Player, PlayerDefaults};
use crate::game::store::Store;
use anyhow::Result;

pub mod fields_desk;
pub mod indexes;
pub mod main_desk;
pub mod player;
pub mod store;

#[derive(Clone)]
pub struct Game {
    pub main_desk: MainDesk,
    pub field_desk: FieldDesk,
    pub indexes: Indexes,
    pub players: Vec<Player>,
    pub store: Store,
    pub prices: StorePrices,
}

impl Game {
    pub fn calculate_final_vp(&mut self) -> Result<()> {
        info!("===========================================");
        info!("Расчёт очков победителя");
        let vp = VictoryPointsConfig::load_from_config()?;
        let mut winner_vp = 0;
        let mut winner_name: &str = "";
        for player in self.players.iter_mut() {
            player.vp =
                vp.calculate_vp(&player, &self.main_desk, &self.field_desk, &self.indexes) as u8;
            if player.vp > winner_vp {
                winner_vp = player.vp;
                winner_name = player.name.as_str();
            }
            info!("Игрок: {} с {} очками", player.name, player.vp);
        }
        info!("Победитель: {} с {} очками", winner_name, winner_vp);
        Ok(())
    }

    pub fn new(players_names: Vec<String>) -> Result<Self> {
        let main_desk = MainDesk::new()?;
        let store = Store::new()?;
        let indexes = Indexes::new()?;
        let field_desk = FieldDesk::new()?;
        let prices = StorePrices::new()?;
        let mut players = vec![];
        for name in players_names {
            players.push(Player::new(name));
        }

        let mut new_game = Game {
            main_desk,
            field_desk,
            indexes,
            players,
            store,
            prices,
        };
        new_game.give_defaults();
        Ok(new_game)
    }

    pub fn get_player_by_hash_mut(&mut self, hash: u64) -> &mut Player {
        self.players
            .iter_mut()
            .find(|player| player.hash == hash)
            .unwrap()
    }

    pub fn get_player_by_hash(&self, hash: u64) -> &Player {
        self.players
            .iter()
            .find(|player| player.hash == hash)
            .unwrap()
    }

    pub fn get_players_hashes(&self) -> Vec<u64> {
        self.players.iter().map(|p| p.hash).collect()
    }

    pub fn give_defaults(&mut self) -> Result<()> {
        let defaults = PlayerDefaults::load_defaults()?;
        for player_hash in self.get_players_hashes() {
            self.give_cube_resource(player_hash, CubeResourceTypeEnum::Wood, defaults.wood);
            self.give_cube_resource(player_hash, CubeResourceTypeEnum::Food, defaults.food);
            self.give_cube_resource(player_hash, CubeResourceTypeEnum::Metal, defaults.metal);
            self.give_cube_resource(player_hash, CubeResourceTypeEnum::Weapon, defaults.weapon);
            self.give_cube_resource(player_hash, CubeResourceTypeEnum::Wax, defaults.wax);
            self.give_cube_resource(player_hash, CubeResourceTypeEnum::Wool, defaults.wool);
            self.give_money(player_hash, defaults.money);
            self.give_reputation(player_hash, defaults.reputation);
            self.give_people(player_hash, defaults.people);
        }
        Ok(())
    }

    pub fn give_cube_resource(
        &mut self,
        player_hash: u64,
        resource: CubeResourceTypeEnum,
        qty: u8,
    ) {
        // Считаем фактическое количество до заимствования player
        let actual = match resource {
            CubeResourceTypeEnum::Food => qty.min(self.store.food),
            CubeResourceTypeEnum::Wood => qty.min(self.store.wood),
            CubeResourceTypeEnum::Metal => qty.min(self.store.metal),
            CubeResourceTypeEnum::Wax => qty.min(self.store.wax),
            CubeResourceTypeEnum::Wool => qty.min(self.store.wool),
            CubeResourceTypeEnum::Weapon => qty.min(self.store.weapon),
        };

        let player = self
            .players
            .iter_mut()
            .find(|p| p.hash == player_hash)
            .unwrap();

        game_info!(
            "Игроку {} дали {} ресурса {}",
            player.name,
            actual,
            resource.name()
        );
        match resource {
            CubeResourceTypeEnum::Food => player.food = player.food.saturating_add(actual),
            CubeResourceTypeEnum::Wood => player.wood = player.wood.saturating_add(actual),
            CubeResourceTypeEnum::Metal => player.metal = player.metal.saturating_add(actual),
            CubeResourceTypeEnum::Wax => player.wax = player.wax.saturating_add(actual),
            CubeResourceTypeEnum::Wool => player.wool = player.wool.saturating_add(actual),
            CubeResourceTypeEnum::Weapon => player.weapon = player.weapon.saturating_add(actual),
        }

        // Уменьшаем запас хранилища
        match resource {
            CubeResourceTypeEnum::Food => self.store.food -= actual,
            CubeResourceTypeEnum::Wood => self.store.wood -= actual,
            CubeResourceTypeEnum::Metal => self.store.metal -= actual,
            CubeResourceTypeEnum::Wax => self.store.wax -= actual,
            CubeResourceTypeEnum::Wool => self.store.wool -= actual,
            CubeResourceTypeEnum::Weapon => self.store.weapon -= actual,
        }
    }

    pub fn take_cube_resource(
        &mut self,
        player_hash: u64,
        resource: CubeResourceTypeEnum,
        qty: u8,
    ) {
        let player = self
            .players
            .iter_mut()
            .find(|p| p.hash == player_hash)
            .unwrap();

        let actual = match resource {
            CubeResourceTypeEnum::Food => qty.min(player.food),
            CubeResourceTypeEnum::Wood => qty.min(player.wood),
            CubeResourceTypeEnum::Metal => qty.min(player.metal),
            CubeResourceTypeEnum::Wax => qty.min(player.wax),
            CubeResourceTypeEnum::Wool => qty.min(player.wool),
            CubeResourceTypeEnum::Weapon => qty.min(player.weapon),
        };

        game_info!(
            "У игрока {} взяли {} ресурса {}",
            player.name,
            actual,
            resource.name()
        );

        match resource {
            CubeResourceTypeEnum::Food => player.food -= actual,
            CubeResourceTypeEnum::Wood => player.wood -= actual,
            CubeResourceTypeEnum::Metal => player.metal -= actual,
            CubeResourceTypeEnum::Wax => player.wax -= actual,
            CubeResourceTypeEnum::Wool => player.wool -= actual,
            CubeResourceTypeEnum::Weapon => player.weapon -= actual,
        }

        // Возвращаем ресурс в хранилище
        match resource {
            CubeResourceTypeEnum::Food => self.store.food = self.store.food.saturating_add(actual),
            CubeResourceTypeEnum::Wood => self.store.wood = self.store.wood.saturating_add(actual),
            CubeResourceTypeEnum::Metal => {
                self.store.metal = self.store.metal.saturating_add(actual)
            }
            CubeResourceTypeEnum::Wax => self.store.wax = self.store.wax.saturating_add(actual),
            CubeResourceTypeEnum::Wool => self.store.wool = self.store.wool.saturating_add(actual),
            CubeResourceTypeEnum::Weapon => {
                self.store.weapon = self.store.weapon.saturating_add(actual)
            }
        }
    }

    pub fn give_money(&mut self, player_hash: u64, qty: u16) {
        let actual = qty.min(self.store.money);
        self.store.money -= actual;
        let player = self
            .players
            .iter_mut()
            .find(|p| p.hash == player_hash)
            .unwrap();
        player.money = player.money.saturating_add(actual);
        game_info!("Игроку {} дали {} монет", player.name, actual);
    }

    pub fn take_money(&mut self, player_hash: u64, qty: u16) {
        let player = self
            .players
            .iter_mut()
            .find(|p| p.hash == player_hash)
            .unwrap();
        let actual = qty.min(player.money);
        player.money -= actual;
        game_info!("У игрока {} взяли {} монет", player.name, actual);
        self.store.money = self.store.money.saturating_add(actual);
    }

    pub fn give_vp(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        player.vp = player.vp.saturating_add(qty);
        game_info!("Игроку {} дали {} ПО", player.name, qty);
    }

    pub fn take_vp(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        player.vp = player.vp.saturating_sub(qty);
        game_info!("У игрока {} взяли {} ПО", player.name, qty);
    }

    pub fn give_reputation(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        player.reputation = player.reputation.saturating_add(qty);
        game_info!("Игроку {} дали {} славы", player.name, qty);
    }

    pub fn take_reputation(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        player.reputation = player.reputation.saturating_sub(qty);
        game_info!("У игрока {} взяли {} славы", player.name, qty);
    }

    pub fn give_people(&mut self, player_hash: u64, qty: u8) {
        let actual = qty.min(self.store.people);
        self.store.people -= actual;
        let player = self.get_player_by_hash_mut(player_hash);
        player.people = player.people.saturating_add(actual);
        game_info!("Игроку {} дали {} людей", player.name, actual);
    }

    pub fn take_people(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        let actual = qty.min(player.people);
        player.people -= actual;
        game_info!("У игрока {} взяли {} людей", player.name, actual);
        self.store.people = self.store.people.saturating_add(actual);
    }

    /// Количество свободных (незанятых) фигурок у игрока.
    /// Свободные = player.people - расставленные на мастерских - расставленные на полях.
    pub fn count_free_people(&self, player_hash: u64) -> u8 {
        let total = self.get_player_by_hash(player_hash).people;
        let on_workshops = self.main_desk.count_player_miples(player_hash);
        let on_fields = self.field_desk.count_player_miples(player_hash);
        total.saturating_sub(on_workshops + on_fields)
    }

    pub fn to_string(&self) -> String {
        format!(
            "Статус игры:\n\
            │ Хранилище:\n\
            {store}\n\
            │ Игроки:\n\
            {players}",
            store = self.store.to_string(),
            players = self
                .players
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
