use crate::data::future::Future;
use crate::data::needs::Needs;
use crate::data::resource::CubeResourceTypeEnum;
use crate::data::workshop::WorkshopTypeEnum;
use crate::game::fields_desk::FieldPlaceTypeEnum;
use crate::game::Game;
use crate::game_info;
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

    /// Уникальный идентификатор конкретного варианта хода (для дедупликации комбинаций)
    pub fn variant_id(&self) -> u8 {
        match self {
            TurnEnum::Build(WorkshopTypeEnum::Weapons) => 0,
            TurnEnum::Build(WorkshopTypeEnum::Feeds) => 1,
            TurnEnum::Build(WorkshopTypeEnum::Waxes) => 2,
            TurnEnum::Build(WorkshopTypeEnum::Wools) => 3,
            TurnEnum::Build(WorkshopTypeEnum::Church) => 4,
            TurnEnum::Build(WorkshopTypeEnum::School) => 5,
            TurnEnum::TakeOverTheWorkshop => 6,
            TurnEnum::TakeOverTheField => 7,
            TurnEnum::PlaceMiples => 8,
            TurnEnum::PlaceResourcesToWorkshop => 9,
            TurnEnum::PlaceMipleToWall => 10,
            TurnEnum::PlayReputationCard => 11,
            TurnEnum::PlayActionCard => 12,
            TurnEnum::Pass1 => 13,
            TurnEnum::Pass2 => 14,
            TurnEnum::Pass3 => 15,
            TurnEnum::BuyActionCard => 16,
            TurnEnum::BuyWarCard => 17,
            TurnEnum::BuyLawCard => 18,
            TurnEnum::Attack => 19,
            TurnEnum::BuyFiled(FieldPlaceTypeEnum::Wood) => 20,
            TurnEnum::BuyFiled(FieldPlaceTypeEnum::Food) => 21,
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wood) => 22,
            TurnEnum::BuyResource(CubeResourceTypeEnum::Food) => 23,
            TurnEnum::BuyResource(CubeResourceTypeEnum::Metal) => 24,
            TurnEnum::BuyResource(CubeResourceTypeEnum::Weapon) => 25,
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wax) => 26,
            TurnEnum::BuyResource(CubeResourceTypeEnum::Wool) => 27,
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wood) => 28,
            TurnEnum::OrderResource(CubeResourceTypeEnum::Food) => 29,
            TurnEnum::OrderResource(CubeResourceTypeEnum::Metal) => 30,
            TurnEnum::OrderResource(CubeResourceTypeEnum::Weapon) => 31,
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wax) => 32,
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wool) => 33,
        }
    }

    pub fn generate_unique_combinations() -> Vec<Vec<TurnEnum>> {
        // Only include turns that are currently implemented
        let all_turns = vec![
            // Build variants
            TurnEnum::Build(WorkshopTypeEnum::Weapons),
            TurnEnum::Build(WorkshopTypeEnum::Feeds),
            TurnEnum::Build(WorkshopTypeEnum::Waxes),
            TurnEnum::Build(WorkshopTypeEnum::Wools),
            TurnEnum::Build(WorkshopTypeEnum::Church),
            TurnEnum::Build(WorkshopTypeEnum::School),
            // Takeover
            TurnEnum::TakeOverTheWorkshop,
            // Pass variants
            TurnEnum::Pass1,
            TurnEnum::Pass2,
            TurnEnum::Pass3,
            // Buy cards
            TurnEnum::BuyActionCard,
            TurnEnum::BuyWarCard,
            TurnEnum::BuyLawCard,
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
            // Расстановка фигурок и ресурсов
            TurnEnum::PlaceMiples,
            TurnEnum::PlaceResourcesToWorkshop,
            // Выставление ресурсов на рынок
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wood),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Food),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Metal),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Weapon),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wax),
            TurnEnum::OrderResource(CubeResourceTypeEnum::Wool),
        ];

        let mut combinations = Vec::new();
        let mut seen_combinations: HashSet<(u8, u8, u8)> = HashSet::new();

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

                    // Create combination and check for uniqueness by concrete variant
                    let variant_tuple =
                        (turn1.variant_id(), turn2.variant_id(), turn3.variant_id());

                    if seen_combinations.insert(variant_tuple) {
                        combinations.push(vec![turn1.clone(), turn2.clone(), turn3.clone()]);
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
    needs: &mut Needs,
) -> bool {
    match turn {
        TurnEnum::Build(workshop_type) => {
            build_workshop(game, player_hash, player_future, needs, workshop_type)
        }
        TurnEnum::BuyFiled(field_type) => buy_field(game, player_hash, needs, field_type),
        TurnEnum::BuyResource(resource_type) => {
            buy_resource(game, player_hash, needs, resource_type)
        }
        TurnEnum::BuyActionCard => buy_action_card(game, player_hash, needs),
        TurnEnum::BuyWarCard => buy_war_card(game, player_hash, needs),
        TurnEnum::BuyLawCard => buy_law_card(game, player_hash, needs),
        TurnEnum::Pass1 => make_pass(game, player_hash),
        TurnEnum::Pass2 => make_pass(game, player_hash),
        TurnEnum::Pass3 => make_pass(game, player_hash),
        TurnEnum::TakeOverTheWorkshop => take_over_workshop(game, player_hash, needs),
        TurnEnum::TakeOverTheField => false, // Правилами запрещено
        TurnEnum::PlaceMiples => place_miples(game, player_hash, needs),
        TurnEnum::PlaceResourcesToWorkshop => {
            place_resources_to_workshop(game, player_hash, player_future, needs)
        }
        TurnEnum::PlaceMipleToWall => false, // Нет в правилах
        TurnEnum::OrderResource(resource_type) => {
            order_resource(game, player_hash, needs, resource_type)
        }
        TurnEnum::PlayReputationCard => false, // Исключено из реализации
        TurnEnum::PlayActionCard => false,     // Исключено из реализации
        TurnEnum::Attack => false,             // Война пока игнорируется
    }
}

fn make_pass(game: &mut Game, player_hash: u64) -> bool {
    game.give_money(player_hash, 2);
    true
}

/// Строительство мастерской: проверяем наличие места, ресурсов у игрока, запас мастерских в хранилище.
/// Списываем ресурсы, ставим мастерскую, даём +1 репутацию и +2 VP.
fn build_workshop(
    game: &mut Game,
    player_hash: u64,
    player_future: &mut Future,
    needs: &mut Needs,
    workshop_type: WorkshopTypeEnum,
) -> bool {
    // Проверяем запас мастерских в хранилище
    if !game.store.has_workshop(&workshop_type) {
        return false;
    }

    // Ищем свободное место (предпочитаем со свободными соседями)
    let free_place = game
        .main_desk
        .get_free_place_with_free_nhs()
        .or_else(|| game.main_desk.get_free_place());

    let free_place_id = match free_place {
        Some(id) => id,
        None => return false,
    };

    // Проверяем, может ли игрок заплатить
    let price = game.prices.get_workshop_price(&workshop_type).clone();
    let player = game.get_player_by_hash(player_hash);
    if !price.can_afford(player) {
        price.record_needs(needs);
        return false;
    }

    // Списываем ресурсы
    price.deduct(game, player_hash);

    // Уменьшаем запас мастерских
    game.store.take_workshop(&workshop_type);

    // Ставим мастерскую на место
    let _ =
        game.main_desk
            .set_workshop_and_owner(free_place_id, player_hash, workshop_type.clone());

    // Если есть свободные фигурки — сразу занимаем место
    if game.count_free_people(player_hash) > 0 {
        game.main_desk.set_miple(free_place_id);
    }

    // Награда: +1 репутация, +2 VP
    game.give_reputation(player_hash, 1);
    game.give_vp(player_hash, 2);

    // Прогнозируем будущее по типу мастерской
    match workshop_type {
        WorkshopTypeEnum::Feeds => {
            player_future.money += 5;
        }
        WorkshopTypeEnum::Weapons => {
            player_future.weapon += 1;
        }
        WorkshopTypeEnum::Waxes => {
            player_future.wax += 1;
        }
        WorkshopTypeEnum::Wools => {
            player_future.wool += 1;
        }
        WorkshopTypeEnum::Church => {
            player_future.reputation += 1;
        }
        WorkshopTypeEnum::School => {
            player_future.people += 1;
        }
    }

    game_info!(
        "Игрок {} построил мастерскую {}",
        game.get_player_by_hash(player_hash).name,
        workshop_type.name()
    );
    true
}

/// Покупка земли: проверяем наличие свободного поля, ресурсов у игрока.
/// Списываем ресурсы, покупаем поле, даём +1 репутацию и +2 VP.
fn buy_field(
    game: &mut Game,
    player_hash: u64,
    needs: &mut Needs,
    field_type: FieldPlaceTypeEnum,
) -> bool {
    // Ищем свободное поле нужного типа
    let field_index = match game.field_desk.get_free_field(&field_type) {
        Some(idx) => idx,
        None => return false,
    };

    // Проверяем цену
    let price = game.prices.get_field_price(&field_type).clone();
    let player = game.get_player_by_hash(player_hash);
    if !price.can_afford(player) {
        price.record_needs(needs);
        return false;
    }

    // Списываем ресурсы
    price.deduct(game, player_hash);

    // VP за поле
    let field_vp = game.field_desk.get_field_vp(field_index);

    // Покупаем поле
    game.field_desk.buy_field(field_index, player_hash);

    // Если есть свободные фигурки — сразу занимаем поле
    if game.count_free_people(player_hash) > 0 {
        game.field_desk.set_miple(field_index);
    }

    // Награда: +1 репутация, +VP за поле
    game.give_reputation(player_hash, 1);
    game.give_vp(player_hash, field_vp);

    game_info!(
        "Игрок {} купил землю типа {}",
        game.get_player_by_hash(player_hash).name,
        field_type.name()
    );
    true
}

/// Покупка ресурса с рыночного лотка магазина.
/// Бот покупает 1 единицу ресурса по цене, установленной на лотке.
fn buy_resource(
    game: &mut Game,
    player_hash: u64,
    needs: &mut Needs,
    resource_type: CubeResourceTypeEnum,
) -> bool {
    // Проверяем, есть ли ресурс на лотке магазина
    let stall = game.store.get_stall(&resource_type);
    if stall.qty == 0 {
        return false;
    }
    let stall_price = stall.price;

    // Проверяем, хватает ли денег у игрока
    let player = game.get_player_by_hash(player_hash);
    if player.money < stall_price {
        needs.put_money(stall_price.min(255) as u8);
        return false;
    }

    // Списываем деньги
    game.take_money(player_hash, stall_price);

    // Уменьшаем количество на лотке
    game.store.get_stall_mut(&resource_type).qty -= 1;

    // Даём ресурс игроку (берётся из запасов хранилища)
    game.give_cube_resource(player_hash, resource_type.clone(), 1);

    game_info!(
        "Игрок {} купил ресурс {} с лотка за {}",
        game.get_player_by_hash(player_hash).name,
        resource_type.name(),
        stall_price
    );
    true
}

/// Покупка карты действий
fn buy_action_card(game: &mut Game, player_hash: u64, needs: &mut Needs) -> bool {
    if game.store.action_cards.is_empty() {
        return false;
    }

    let price = game.prices.card_action.clone();
    let player = game.get_player_by_hash(player_hash);
    if !price.can_afford(player) {
        price.record_needs(needs);
        return false;
    }

    price.deduct(game, player_hash);
    let card = game.store.action_cards.pop().unwrap();
    game.get_player_by_hash_mut(player_hash)
        .action_cards
        .push(card);

    game_info!(
        "Игрок {} купил карту действий",
        game.get_player_by_hash(player_hash).name
    );
    true
}

/// Покупка карты закона
fn buy_law_card(game: &mut Game, player_hash: u64, needs: &mut Needs) -> bool {
    if game.store.law_cards.is_empty() {
        return false;
    }

    let price = game.prices.card_law.clone();
    let player = game.get_player_by_hash(player_hash);
    if !price.can_afford(player) {
        price.record_needs(needs);
        return false;
    }

    price.deduct(game, player_hash);
    let card = game.store.law_cards.pop().unwrap();
    game.get_player_by_hash_mut(player_hash)
        .law_cards
        .push(card);

    game_info!(
        "Игрок {} купил карту закона",
        game.get_player_by_hash(player_hash).name
    );
    true
}

/// Покупка военной карты (случайный тип: пехота/конница/лучники)
fn buy_war_card(game: &mut Game, player_hash: u64, needs: &mut Needs) -> bool {
    // Проверяем, есть ли хоть одна военная карта
    let total_war = game.store.war_cards_infantry.len()
        + game.store.war_cards_cavalry.len()
        + game.store.war_cards_archer.len();
    if total_war == 0 {
        return false;
    }

    let price = game.prices.card_war.clone();
    let player = game.get_player_by_hash(player_hash);
    if !price.can_afford(player) {
        price.record_needs(needs);
        return false;
    }

    price.deduct(game, player_hash);

    // Берём первую доступную карту по приоритету: пехота -> конница -> лучники
    if let Some(card) = game.store.war_cards_infantry.pop() {
        game.get_player_by_hash_mut(player_hash)
            .war_cards_infantry
            .push(card);
    } else if let Some(card) = game.store.war_cards_cavalry.pop() {
        game.get_player_by_hash_mut(player_hash)
            .war_cards_cavalry
            .push(card);
    } else if let Some(card) = game.store.war_cards_archer.pop() {
        game.get_player_by_hash_mut(player_hash)
            .war_cards_archer
            .push(card);
    }

    game_info!(
        "Игрок {} купил военную карту",
        game.get_player_by_hash(player_hash).name
    );
    true
}

/// Захват мастерской другого игрока (правило 6.4).
/// Стоимость: 5 монет посаднику (или в пул), 5 монет текущему владельцу.
/// Условия: цель имеет фишку (не фигурку), у атакующего есть 10 монет.
fn take_over_workshop(game: &mut Game, player_hash: u64, needs: &mut Needs) -> bool {
    // Ищем цель для захвата
    let target = game.main_desk.find_takeover_target(player_hash);
    let (place_id, current_owner_hash) = match target {
        Some(t) => t,
        None => return false,
    };

    // Проверяем, хватает ли денег (10 монет)
    let player = game.get_player_by_hash(player_hash);
    if player.money < 10 {
        needs.put_money(10);
        return false;
    }

    // Списываем 10 монет у атакующего
    game.take_money(player_hash, 10);

    // 5 монет посаднику (если позиция занята и это не сам атакующий)
    let mayor_hash = game.main_desk.get_mayor_hash();
    if mayor_hash != 0 && mayor_hash != player_hash {
        game.give_money(mayor_hash, 5);
    } else {
        // Если посадника нет — деньги уходят в пул (store)
        // money уже в store после take_money, нужно вернуть 5 обратно
        // take_money уже вернул 10 в store, give_money заберёт из store
        // Ничего делать не нужно — 5 монет остаются в store
    }

    // 5 монет текущему владельцу мастерской
    game.give_money(current_owner_hash, 5);

    // Захватываем мастерскую
    game.main_desk.takeover_workshop(place_id, player_hash);

    game_info!(
        "Игрок {} захватил мастерскую (место {})",
        game.get_player_by_hash(player_hash).name,
        place_id
    );
    true
}

/// Расстановка фигурок (правило 6.12).
/// Бот автоматически расставляет свободные фигурки на мастерские без фигурок.
/// Приоритет: Едальня > Кузня > Церковь > Школа > Восковая > Кожевенная.
/// Также ставит фигурки на поля без фигурок.
fn place_miples(game: &mut Game, player_hash: u64, needs: &mut Needs) -> bool {
    let player = game.get_player_by_hash(player_hash);
    let total_people = player.people;

    // Считаем сколько фигурок уже расставлено
    let miples_on_workshops = game.main_desk.count_player_miples(player_hash);
    let miples_on_fields = game.field_desk.count_player_miples(player_hash);
    let placed = miples_on_workshops + miples_on_fields;

    if total_people <= placed {
        needs.put_people(1); // Нет свободных фигурок — нужны люди
        return false;
    }

    let mut free_miples = total_people - placed;
    let mut any_placed = false;

    // Собираем мастерские без фигурок, сортируем по приоритету
    let chip_places = game.main_desk.get_player_chip_places(player_hash);
    let mut prioritized: Vec<(u8, u8)> = Vec::new(); // (place_id, priority)

    for place_id in &chip_places {
        let places = game.main_desk.get_player_places(player_hash);
        if let Some(place) = places.iter().find(|p| p.id == *place_id) {
            let priority = match &place.workshop_type {
                Some(WorkshopTypeEnum::Feeds) => 0,
                Some(WorkshopTypeEnum::Weapons) => 1,
                Some(WorkshopTypeEnum::Church) => 2,
                Some(WorkshopTypeEnum::School) => 3,
                Some(WorkshopTypeEnum::Waxes) => 4,
                Some(WorkshopTypeEnum::Wools) => 5,
                None => 10,
            };
            prioritized.push((*place_id, priority));
        }
    }
    prioritized.sort_by_key(|&(_, prio)| prio);

    // Ставим фигурки на мастерские
    for (place_id, _) in prioritized {
        if free_miples == 0 {
            break;
        }
        game.main_desk.set_miple(place_id);
        free_miples -= 1;
        any_placed = true;
    }

    // Ставим фигурки на поля
    let chip_fields = game.field_desk.get_player_chip_fields(player_hash);
    for field_idx in chip_fields {
        if free_miples == 0 {
            break;
        }
        game.field_desk.set_miple(field_idx);
        free_miples -= 1;
        any_placed = true;
    }

    if any_placed {
        game_info!(
            "Игрок {} расставил фигурки",
            game.get_player_by_hash(player_hash).name
        );
    }
    any_placed
}

/// Размещение ресурсов в мастерские (правило 6.1).
/// Это ход-«заявка»: бот проверяет, есть ли у него мастерские с фигурками
/// и ресурсы в руке для производства. Фактическое потребление ресурсов
/// происходит в month_action_workshop во время месячной фазы.
/// Ход возвращает true, если хотя бы одна мастерская может быть обеспечена.
fn place_resources_to_workshop(
    game: &mut Game,
    player_hash: u64,
    _player_future: &mut Future,
    needs: &mut Needs,
) -> bool {
    // Получаем список мастерских игрока с фигурками
    let player_places = game.main_desk.get_player_places(player_hash);
    let workshop_infos: Vec<WorkshopTypeEnum> = player_places
        .iter()
        .filter(|place| place.is_miple && place.workshop_type.is_some())
        .filter_map(|place| place.workshop_type.clone())
        .collect();

    if workshop_infos.is_empty() {
        return false;
    }

    // Проверяем, есть ли ресурсы хотя бы для одной мастерской
    let player = game.get_player_by_hash(player_hash);
    let mut any_can_supply = false;
    for wt in &workshop_infos {
        let can_supply = match wt {
            WorkshopTypeEnum::Feeds => player.food >= 1,
            WorkshopTypeEnum::Weapons => player.metal >= 1,
            WorkshopTypeEnum::Church => player.wax >= 1,
            WorkshopTypeEnum::School => player.wood >= 1 && player.food >= 1,
            // Восковая и кожевенная не требуют ресурсов
            WorkshopTypeEnum::Waxes => true,
            WorkshopTypeEnum::Wools => true,
        };
        if can_supply {
            any_can_supply = true;
        } else {
            // Записываем потребность в недостающих ресурсах
            match wt {
                WorkshopTypeEnum::Feeds => needs.put_food(1),
                WorkshopTypeEnum::Weapons => needs.put_metal(1),
                WorkshopTypeEnum::Church => needs.put_wax(1),
                WorkshopTypeEnum::School => {
                    if player.wood < 1 {
                        needs.put_wood(1);
                    }
                    if player.food < 1 {
                        needs.put_food(1);
                    }
                }
                WorkshopTypeEnum::Waxes => {}
                WorkshopTypeEnum::Wools => {}
            }
        }
    }
    if any_can_supply {
        game_info!(
            "Игрок {} подготовил ресурсы для мастерских",
            game.get_player_by_hash(player_hash).name
        );
    }
    any_can_supply
}

/// Выставить ресурс на продажу на рынке (правило 6.10).
/// Бот выставляет 1 единицу ресурса по средней цене.
fn order_resource(
    game: &mut Game,
    player_hash: u64,
    needs: &mut Needs,
    resource_type: CubeResourceTypeEnum,
) -> bool {
    // Проверяем, есть ли ресурс у игрока
    let player = game.get_player_by_hash(player_hash);
    let has_resource = match &resource_type {
        CubeResourceTypeEnum::Wood => player.wood >= 1,
        CubeResourceTypeEnum::Food => player.food >= 1,
        CubeResourceTypeEnum::Metal => player.metal >= 1,
        CubeResourceTypeEnum::Weapon => player.weapon >= 1,
        CubeResourceTypeEnum::Wax => player.wax >= 1,
        CubeResourceTypeEnum::Wool => player.wool >= 1,
    };
    if !has_resource {
        // Записываем потребность в этом ресурсе
        match &resource_type {
            CubeResourceTypeEnum::Wood => needs.put_wood(1),
            CubeResourceTypeEnum::Food => needs.put_food(1),
            CubeResourceTypeEnum::Metal => needs.put_metal(1),
            CubeResourceTypeEnum::Weapon => needs.put_weapon(1),
            CubeResourceTypeEnum::Wax => needs.put_wax(1),
            CubeResourceTypeEnum::Wool => needs.put_wool(1),
        }
        return false;
    }

    // Проверяем, не занят ли уже лоток
    let stall = player.get_stall(&resource_type);
    if stall.qty >= 6 {
        return false; // Лоток заполнен
    }

    // Средняя цена для ресурса из конфига
    let range = game.prices.get_resource_range(&resource_type);
    let price = range.average();

    // Забираем ресурс у игрока (НЕ возвращаем в store — ресурс на лотке)
    let player = game.get_player_by_hash_mut(player_hash);
    match &resource_type {
        CubeResourceTypeEnum::Wood => player.wood -= 1,
        CubeResourceTypeEnum::Food => player.food -= 1,
        CubeResourceTypeEnum::Metal => player.metal -= 1,
        CubeResourceTypeEnum::Weapon => player.weapon -= 1,
        CubeResourceTypeEnum::Wax => player.wax -= 1,
        CubeResourceTypeEnum::Wool => player.wool -= 1,
    };
    let stall = player.get_stall_mut(&resource_type);
    stall.qty += 1;
    stall.price = price;

    game_info!(
        "Игрок {} выставил {} на рынок по цене {}",
        game.get_player_by_hash(player_hash).name,
        resource_type.name(),
        price
    );
    true
}
