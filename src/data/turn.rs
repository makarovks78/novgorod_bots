use crate::data::future::Future;
use crate::data::resource::CubeResourceTypeEnum;
use crate::data::workshop::WorkshopTypeEnum;
use crate::game::Game;
use crate::game::fields_desk::FieldPlaceTypeEnum;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum TurnEnum {
    Build(WorkshopTypeEnum),
    TakeOverTheWorkshop,
    TakeOverTheField,
    PlaceMiples,
    PlaceResourcesToWorkshop,
    PlaceMipleToWall,
    BuyFiled(FieldPlaceTypeEnum),
    BuyResource(CubeResourceTypeEnum),
    OrderResource(CubeResourceTypeEnum),
    PlayReputationCard,
    PlayActionCard,
    Pass1,
    Pass2,
    Pass3,
    BuyActionCard,
    BuyWarCard,
    BuyLawCard,
    Attack,
}

impl TurnEnum {
    pub fn get_type(&self) -> TurnTypeEnum {
        match self {
            TurnEnum::Build(_) => TurnTypeEnum::Build,
            TurnEnum::TakeOverTheWorkshop => TurnTypeEnum::TakeOver,
            TurnEnum::TakeOverTheField => TurnTypeEnum::TakeOver,
            TurnEnum::PlaceMiples => TurnTypeEnum::PlaceMiples,
            TurnEnum::PlaceResourcesToWorkshop => TurnTypeEnum::PlaceResources,
            TurnEnum::PlaceMipleToWall => TurnTypeEnum::PlaceMipleToWall,
            TurnEnum::BuyFiled(_) => TurnTypeEnum::BuyField,
            TurnEnum::BuyResource(_) => TurnTypeEnum::BuyResource,
            TurnEnum::OrderResource(_) => TurnTypeEnum::OrderResource,
            TurnEnum::PlayReputationCard => TurnTypeEnum::PlayReputationCard,
            TurnEnum::PlayActionCard => TurnTypeEnum::PlayActionCard,
            TurnEnum::Pass1 => TurnTypeEnum::Pass1,
            TurnEnum::Pass2 => TurnTypeEnum::Pass2,
            TurnEnum::Pass3 => TurnTypeEnum::Pass3,
            TurnEnum::BuyActionCard => TurnTypeEnum::BuyActionCard,
            TurnEnum::BuyWarCard => TurnTypeEnum::BuyWarCard,
            TurnEnum::BuyLawCard => TurnTypeEnum::BuyLawCard,
            TurnEnum::Attack => TurnTypeEnum::Attack,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TurnEnum::Build(WorkshopTypeEnum::Weapons) => "Build Weapons".to_string(),
            TurnEnum::Build(WorkshopTypeEnum::Feeds) => "Build Feeds".to_string(),
            TurnEnum::Build(WorkshopTypeEnum::Waxes) => "Build Waxes".to_string(),
            TurnEnum::Build(WorkshopTypeEnum::Wools) => "Build Wools".to_string(),
            TurnEnum::Build(WorkshopTypeEnum::Church) => "Build Church".to_string(),
            TurnEnum::Build(WorkshopTypeEnum::School) => "Build School".to_string(),
            TurnEnum::TakeOverTheWorkshop => "Take Over The Workshop".to_string(),
            TurnEnum::TakeOverTheField => "Take Over The Field".to_string(),
            TurnEnum::PlaceMiples => "Place Miples".to_string(),
            TurnEnum::PlaceResourcesToWorkshop => "Place Resources To Workshop".to_string(),
            TurnEnum::PlaceMipleToWall => "Place Miple To Wall".to_string(),
            TurnEnum::PlayReputationCard => "Play Reputation Card".to_string(),
            TurnEnum::PlayActionCard => "Play Action Card".to_string(),
            TurnEnum::Pass1 => "Pass 1".to_string(),
            TurnEnum::Pass2 => "Pass 2".to_string(),
            TurnEnum::Pass3 => "Pass 3".to_string(),
            TurnEnum::BuyActionCard => "Buy Action Card".to_string(),
            TurnEnum::BuyWarCard => "Buy War Card".to_string(),
            TurnEnum::BuyLawCard => "Buy Law Card".to_string(),
            TurnEnum::Attack => "Attack".to_string(),
            TurnEnum::BuyFiled(FieldPlaceTypeEnum::Wood) => "Buy Filed Wood".to_string(),
            TurnEnum::BuyFiled(FieldPlaceTypeEnum::Food) => "Buy Filed Food".to_string(),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wood) => "Buy Resource Wood".to_string(),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Food) => "Buy Resource Food".to_string(),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Metal) => "Buy Resource Metal".to_string(),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Weapon) => {
                "Buy Resource Weapon".to_string()
            }
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wax) => "Buy Resource Wax".to_string(),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wool) => "Buy Resource Wool".to_string(),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wood) => {
                "Order Resource Wood".to_string()
            }
            TurnEnum::OrderResource(CubeResourceTypeEnum::Food) => {
                "Order Resource Food".to_string()
            }
            TurnEnum::OrderResource(CubeResourceTypeEnum::Metal) => {
                "Order Resource Metal".to_string()
            }
            TurnEnum::OrderResource(CubeResourceTypeEnum::Weapon) => {
                "Order Resource Weapon".to_string()
            }
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wax) => "Order Resource Wax".to_string(),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wool) => {
                "Order Resource Wool".to_string()
            }
        }
    }

    pub fn generate_unique_combinations() -> Vec<Vec<TurnEnum>> {
        // Create concrete instances of all possible turns
        let all_turns = vec![
            // Build variants
            TurnEnum::Build(WorkshopTypeEnum::Weapons),
            TurnEnum::Build(WorkshopTypeEnum::Feeds),
            TurnEnum::Build(WorkshopTypeEnum::Waxes),
            TurnEnum::Build(WorkshopTypeEnum::Wools),
            TurnEnum::Build(WorkshopTypeEnum::Church),
            TurnEnum::Build(WorkshopTypeEnum::School),
            // Simple variants without parameters
            TurnEnum::TakeOverTheWorkshop,
            TurnEnum::TakeOverTheField,
            TurnEnum::PlaceMiples,
            TurnEnum::PlaceResourcesToWorkshop,
            TurnEnum::PlaceMipleToWall,
            TurnEnum::PlayReputationCard,
            TurnEnum::PlayActionCard,
            TurnEnum::Pass1,
            TurnEnum::Pass2,
            TurnEnum::Pass3,
            TurnEnum::BuyActionCard,
            TurnEnum::BuyWarCard,
            TurnEnum::BuyLawCard,
            TurnEnum::Attack,
            // BuyFiled variants
            TurnEnum::BuyFiled(FieldPlaceTypeEnum::Wood),
            TurnEnum::BuyFiled(FieldPlaceTypeEnum::Food),
            // BuyResource variants
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wood),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Food),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Metal),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Weapon),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wax),
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wool),
            // OrderResource variants
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wood),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Food),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Metal),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Weapon),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wax),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wool),
        ];

        let mut combinations = Vec::new();
        let mut seen_combinations = HashSet::new();

        for i in 0..all_turns.len() {
            for j in 0..all_turns.len() {
                for k in 0..all_turns.len() {
                    let turn1 = &all_turns[i];
                    let turn2 = &all_turns[j];
                    let turn3 = &all_turns[k];

                    // Check Pass position constraints
                    if matches!(turn1, TurnEnum::Pass2 | TurnEnum::Pass3) {
                        continue;
                    }
                    if matches!(turn2, TurnEnum::Pass1 | TurnEnum::Pass3) {
                        continue;
                    }
                    if matches!(turn3, TurnEnum::Pass1 | TurnEnum::Pass2) {
                        continue;
                    }

                    // Check that all three turns have different types
                    let type1 = turn1.get_type();
                    let type2 = turn2.get_type();
                    let type3 = turn3.get_type();

                    if type1 == type2 || type1 == type3 || type2 == type3 {
                        continue;
                    }

                    // Create combination and check for uniqueness
                    let combination = vec![turn1.clone(), turn2.clone(), turn3.clone()];
                    let type_tuple = (type1, type2, type3);

                    if seen_combinations.insert(type_tuple) {
                        combinations.push(combination);
                    }
                }
            }
        }
        combinations
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TurnTypeEnum {
    Build,
    BuyField,
    PlaceMipleToWall,
    BuyActionCard,
    BuyLawCard,
    BuyWarCard,
    TakeOver,
    PlaceMiples,
    PlaceResources,
    Pass1,
    Pass2,
    Pass3,
    BuyResource,
    OrderResource,
    PlayReputationCard,
    PlayActionCard,
    Attack,
}

pub fn make_turn(
    game: &mut Game,
    turn: TurnEnum,
    player_hash: u64,
    player_future: &mut Future,
) -> bool {
    let player = game.get_player_by_hash(player_hash);
    match turn {
        TurnEnum::Build(workshop_type) => {
            return build_workshop(
                game,
                player_hash,
                player_future,
                workshop_type,
            );
        }
        TurnEnum::TakeOverTheWorkshop => false,
        TurnEnum::TakeOverTheField => false,
        TurnEnum::PlaceMiples => false,
        TurnEnum::PlaceResourcesToWorkshop => false,
        TurnEnum::PlaceMipleToWall => false,
        TurnEnum::BuyFiled(field_type) => false,
        TurnEnum::BuyResource(resource_type) => false,
        TurnEnum::OrderResource(resource_type) => false,
        TurnEnum::PlayReputationCard => false,
        TurnEnum::PlayActionCard => false,
        TurnEnum::Pass1 => {
            return make_pass(game, player_hash);
        }
        TurnEnum::Pass2 => {
            return make_pass(game, player_hash);
        }
        TurnEnum::Pass3 => {
            return make_pass(game, player_hash);
        }
        TurnEnum::BuyActionCard => false,
        TurnEnum::BuyWarCard => false,
        TurnEnum::BuyLawCard => false,
        TurnEnum::Attack => false,
    }
}

fn make_pass(
    game: &mut Game,
    player_hash: u64
) -> bool {
    game.give_money(player_hash, 2);
    true
}

fn build_workshop(
    game: &mut Game,
    player_hash: u64,
    player_future: &mut Future,
    workshop_type: WorkshopTypeEnum,
) -> bool {
    let mut free_place = game.main_desk.get_free_place_with_free_nhs();

    if free_place == None {
        free_place = game.main_desk.get_free_place();
    }

    if free_place == None {
        return false;
    }
    let free_place_id = free_place.unwrap();

    match workshop_type {
        WorkshopTypeEnum::Feeds => {
            player_future.food += 10;
            
        }
        _ => {}
    }

    true
}
