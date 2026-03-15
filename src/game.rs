use log::info;

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
        let player = self.get_player_by_hash_mut(player_hash);
        info!(
            "Игроку {} дали {} ресурса {}",
            player.name,
            qty,
            resource.name()
        );
        match resource {
            CubeResourceTypeEnum::Food => {
                player.food += qty;
                self.store.food -= qty;
            }
            CubeResourceTypeEnum::Wood => {
                player.wood += qty;
                self.store.wood -= qty;
            }
            CubeResourceTypeEnum::Metal => {
                player.metal += qty;
                self.store.metal -= qty;
            }
            CubeResourceTypeEnum::Wax => {
                player.wax += qty;
                self.store.wax -= qty;
            }
            CubeResourceTypeEnum::Wool => {
                player.wool += qty;
                self.store.wool -= qty;
            }
            CubeResourceTypeEnum::Weapon => {
                player.weapon += qty;
                self.store.weapon -= qty;
            }
        }
    }

    pub fn take_cube_resource(
        &mut self,
        player_hash: u64,
        resource: CubeResourceTypeEnum,
        qty: u8,
    ) {
        let player = self.get_player_by_hash_mut(player_hash);
        info!(
            "У игрока {} взяли {} ресурса {}",
            player.name,
            qty,
            resource.name()
        );
        match resource {
            CubeResourceTypeEnum::Food => {
                player.food -= qty;
                self.store.food += qty;
            }
            CubeResourceTypeEnum::Wood => {
                player.wood -= qty;
                self.store.wood += qty;
            }
            CubeResourceTypeEnum::Metal => {
                player.metal -= qty;
                self.store.metal += qty;
            }
            CubeResourceTypeEnum::Wax => {
                player.wax -= qty;
                self.store.wax += qty;
            }
            CubeResourceTypeEnum::Wool => {
                player.wool -= qty;
                self.store.wool += qty;
            }
            CubeResourceTypeEnum::Weapon => {
                player.weapon -= qty;
                self.store.weapon += qty;
            }
        }
    }

    pub fn give_money(&mut self, player_hash: u64, qty: u16) {
        self.store.money -= qty;
        let player = self.get_player_by_hash_mut(player_hash);
        player.money += qty;
        info!("Игроку {} дали {} монет", player.name, qty);
    }

    pub fn take_money(&mut self, player_hash: u64, qty: u16) {
        self.store.money += qty;
        let player = self.get_player_by_hash_mut(player_hash);
        player.money -= qty;
        info!("У игрока {} взяли {} монет", player.name, qty);
    }

    pub fn give_vp(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        player.vp += qty;
        info!("Игроку {} дали {} ПО", player.name, qty);
    }

    pub fn take_vp(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        player.vp -= qty;
        info!("У игрока {} взяли {} ПО", player.name, qty);
    }

    pub fn give_reputation(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        player.reputation += qty;
        info!("Игроку {} дали {} славы", player.name, qty);
    }

    pub fn take_reputation(&mut self, player_hash: u64, qty: u8) {
        let player = self.get_player_by_hash_mut(player_hash);
        player.reputation -= qty;
        info!("У игрока {} взяли {} славы", player.name, qty);
    }

    pub fn give_people(&mut self, player_hash: u64, qty: u8) {
        self.store.people -= qty;
        let player = self.get_player_by_hash_mut(player_hash);
        player.people += qty;
        info!("Игроку {} дали {} людей", player.name, qty);
    }

    pub fn take_people(&mut self, player_hash: u64, qty: u8) {
        self.store.people += qty;
        let player = self.get_player_by_hash_mut(player_hash);
        player.people -= qty;
        info!("У игрока {} взяли {} людей", player.name, qty);
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
