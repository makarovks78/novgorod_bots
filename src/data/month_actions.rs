use crate::data::resource::CubeResourceTypeEnum;
use crate::data::workshop::WorkshopTypeEnum;
use crate::game::fields_desk::FieldPlaceTypeEnum;
use crate::game::Game;
use log::info;
use rand::Rng;

/// Война — заглушка (по answers.txt механика войны игнорируется)
pub fn month_action_war(_game: &mut Game) {
    info!("Выполняется действие месяца: война (пропуск)");
}

/// Событие — заглушка (колоды событий пусты в store.toml)
pub fn month_action_event(_game: &mut Game) {
    info!("Выполняется действие месяца: событие (пропуск — колода пуста)");
}

/// Сбор еды с полей: только поля с фигуркой (is_miple) производят ресурсы
pub fn month_action_food(game: &mut Game) {
    info!("Выполняется действие месяца: сбор еды");

    for player_hash in game.get_players_hashes() {
        let food_fields = game
            .field_desk
            .get_player_active_fields_by_type(player_hash, &FieldPlaceTypeEnum::Food);
        let total_food: u8 = food_fields.iter().map(|(_, qty)| qty).sum();
        if total_food > 0 {
            let available = game.store.food;
            let give_qty = total_food.min(available);
            if give_qty > 0 {
                game.give_cube_resource(player_hash, CubeResourceTypeEnum::Food, give_qty);
            }
        }
    }
}

/// Сбор дерева с полей: только поля с фигуркой (is_miple) производят ресурсы
pub fn month_action_wood(game: &mut Game) {
    info!("Выполняется действие месяца: сбор дерева");

    for player_hash in game.get_players_hashes() {
        let wood_fields = game
            .field_desk
            .get_player_active_fields_by_type(player_hash, &FieldPlaceTypeEnum::Wood);
        let total_wood: u8 = wood_fields.iter().map(|(_, qty)| qty).sum();
        if total_wood > 0 {
            let available = game.store.wood;
            let give_qty = total_wood.min(available);
            if give_qty > 0 {
                game.give_cube_resource(player_hash, CubeResourceTypeEnum::Wood, give_qty);
            }
        }
    }
}

/// Прокорм: каждый игрок тратит еду на содержание фигурок.
/// Количество еды определяется по шкале people_food_map.
/// Если еды не хватает — теряет 1 репутацию за каждую недостающую единицу.
pub fn month_action_feed(game: &mut Game) {
    info!("Выполняется действие месяца: прокорм");

    for player_hash in game.get_players_hashes() {
        let player = game.get_player_by_hash(player_hash);
        let people = player.people;
        let food_needed = *game.indexes.people_food_map.get(&people).unwrap_or(&0);

        if food_needed == 0 {
            continue;
        }

        let player = game.get_player_by_hash(player_hash);
        let player_food = player.food;

        if player_food >= food_needed {
            // Хватает еды — списываем
            game.take_cube_resource(player_hash, CubeResourceTypeEnum::Food, food_needed);
        } else {
            // Не хватает еды — списываем всё что есть и теряем репутацию
            let deficit = food_needed - player_food;
            if player_food > 0 {
                game.take_cube_resource(player_hash, CubeResourceTypeEnum::Food, player_food);
            }
            // За каждую недостающую единицу еды -1 репутация
            let player = game.get_player_by_hash(player_hash);
            let rep_loss = deficit.min(player.reputation);
            if rep_loss > 0 {
                game.take_reputation(player_hash, rep_loss);
            }
            info!(
                "Игрок {} не смог прокормить фигурки, потерял {} славы",
                game.get_player_by_hash(player_hash).name,
                rep_loss
            );
        }
    }
}

/// Вече (голосование за лидеров).
/// Бот с наибольшей репутацией предлагает себя на вакантные позиции.
/// Голосование: бросок 5D6, порог зависит от настроения (mood).
/// mood 1 → нужно 18, mood 2 → 17, ..., mood 6 → 13.
/// При провале голосования предложивший теряет 2 славы.
pub fn month_action_vote(game: &mut Game) {
    info!("Выполняется действие месяца: вече");

    let mut rng = rand::rng();

    // Порог голосования по настроению
    let mood = game.store.mood;
    let threshold: u8 = match mood {
        1 => 18,
        2 => 17,
        3 => 16,
        4 => 15,
        5 => 14,
        _ => 13, // mood 6+
    };

    // Определяем, какие позиции вакантны
    let mayor_vacant = game.main_desk.get_mayor_hash() == 0;
    // Для архиепископа и тысяцкого: проверяем через is_player_archbishop/commander с 0
    // (если hash == 0, позиция свободна)

    // Собираем игроков, сортируем по репутации (больше = приоритет)
    let mut player_infos: Vec<(u64, u8)> = game
        .get_players_hashes()
        .iter()
        .map(|&h| {
            let p = game.get_player_by_hash(h);
            (h, p.reputation)
        })
        .collect();
    player_infos.sort_by(|a, b| b.1.cmp(&a.1));

    // Попытка занять позицию посадника
    if mayor_vacant {
        if let Some(&(candidate_hash, _)) = player_infos.first() {
            // Бросаем 5D6
            let roll: u8 = (0..5).map(|_| rng.random_range(1..=6u8)).sum();
            if roll >= threshold {
                game.main_desk.set_mayor(candidate_hash);
                info!(
                    "Игрок {} избран посадником (бросок {} >= порог {})",
                    game.get_player_by_hash(candidate_hash).name,
                    roll,
                    threshold
                );
            } else {
                // Провал — теряем 2 славы
                game.take_reputation(candidate_hash, 2);
                info!(
                    "Игрок {} провалил выборы посадника (бросок {} < порог {}), -2 славы",
                    game.get_player_by_hash(candidate_hash).name,
                    roll,
                    threshold
                );
            }
        }
    }
}

/// Рынок: для каждого из 6 типов ресурсов игра сканирует лотки всех игроков.
/// Офферы группируются по ресурсу и сортируются по цене (самые дешёвые первые).
/// Для каждого ресурса бросается D6 — столько единиц игра хочет купить.
/// Покупки идут с самого дешёвого оффера. Если D6 > qty оффера, весь оффер
/// выкупается и покупка переходит к следующему по цене.
/// При одинаковой цене у нескольких игроков — покупка по 1 единице у каждого
/// по кругу, пока не набрано нужное количество.
pub fn month_action_market(game: &mut Game) {
    info!("Выполняется действие месяца: рынок");

    let mut rng = rand::rng();

    let all_resources = [
        CubeResourceTypeEnum::Wood,
        CubeResourceTypeEnum::Food,
        CubeResourceTypeEnum::Metal,
        CubeResourceTypeEnum::Weapon,
        CubeResourceTypeEnum::Wax,
        CubeResourceTypeEnum::Wool,
    ];

    let player_hashes = game.get_players_hashes();

    for resource in &all_resources {
        // Собираем все офферы игроков для этого ресурса: (player_hash, qty, price)
        let mut offers: Vec<(u64, u8, u16)> = Vec::new();
        for &ph in &player_hashes {
            let stall = game.get_player_by_hash(ph).get_stall(resource);
            if stall.qty > 0 && stall.price > 0 {
                offers.push((ph, stall.qty, stall.price));
            }
        }

        if offers.is_empty() {
            continue;
        }

        // Сортируем по цене (дешёвые первые)
        offers.sort_by_key(|o| o.2);

        // D6 — сколько единиц игра хочет купить этого ресурса
        let mut demand: u8 = rng.random_range(1..=6u8);
        info!("Рынок: спрос на {} — {} шт.", resource.name(), demand);

        // Группируем офферы по ценовым уровням
        let mut price_groups: Vec<(u16, Vec<(u64, u8)>)> = Vec::new();
        for (ph, qty, price) in &offers {
            if let Some(last) = price_groups.last_mut() {
                if last.0 == *price {
                    last.1.push((*ph, *qty));
                    continue;
                }
            }
            price_groups.push((*price, vec![(*ph, *qty)]));
        }

        // Покупаем начиная с самой дешёвой группы
        for (price, group) in &price_groups {
            if demand == 0 {
                break;
            }

            if group.len() == 1 {
                // Один продавец по этой цене — покупаем сколько можем
                let (seller_hash, available) = group[0];
                let buy_qty = demand.min(available);
                // Обновляем лоток продавца и начисляем деньги
                let player = game.get_player_by_hash_mut(seller_hash);
                let stall = player.get_stall_mut(resource);
                stall.qty = stall.qty.saturating_sub(buy_qty);
                let total_payment = (*price as u32) * (buy_qty as u32);
                player.money = player
                    .money
                    .saturating_add(total_payment.min(u16::MAX as u32) as u16);
                demand -= buy_qty;
                info!(
                    "Рынок: куплено {} шт. {} у {} по цене {} (итого {})",
                    buy_qty,
                    resource.name(),
                    game.get_player_by_hash(seller_hash).name,
                    price,
                    total_payment
                );
            } else {
                // Несколько продавцов по одной цене — round-robin по 1 единице
                // Копируем остатки для отслеживания
                let mut remaining: Vec<(u64, u8)> = group.clone();
                loop {
                    if demand == 0 {
                        break;
                    }
                    let mut bought_this_round = false;
                    for item in remaining.iter_mut() {
                        if demand == 0 {
                            break;
                        }
                        if item.1 > 0 {
                            // Покупаем 1 единицу у этого продавца
                            let seller_hash = item.0;
                            let player = game.get_player_by_hash_mut(seller_hash);
                            let stall = player.get_stall_mut(resource);
                            stall.qty = stall.qty.saturating_sub(1);
                            player.money = player.money.saturating_add(*price);
                            item.1 -= 1;
                            demand -= 1;
                            bought_this_round = true;
                            info!(
                                "Рынок: куплено 1 шт. {} у {} по цене {}",
                                resource.name(),
                                game.get_player_by_hash(seller_hash).name,
                                price
                            );
                        }
                    }
                    if !bought_this_round {
                        break; // Все продавцы в группе исчерпаны
                    }
                }
            }
        }
    }

    // Пополнение лотков игры: для каждого ресурса, если лоток пуст,
    // бросаем D6 дважды — количество и порядковый номер цены.
    // Диапазоны цен берутся из конфига (range_wood..range_wool).
    // D6=1 → минимальная цена, D6=6 → максимальная.
    for resource in &all_resources {
        let stall = game.store.get_stall(resource);
        if stall.qty > 0 {
            continue; // Лоток не пуст — пропускаем
        }

        let qty_roll: u8 = rng.random_range(1..=6u8);
        let price_roll: u8 = rng.random_range(1..=6u8);

        // Диапазон цен из конфига
        let range = game.prices.get_resource_range(resource);
        // price_roll=1 → range.min, price_roll=6 → range.max
        let price = range.min + (price_roll - 1) as u16;

        let stall = game.store.get_stall_mut(resource);
        stall.qty = qty_roll;
        stall.price = price;

        info!(
            "Лоток игры {}: {} шт. по цене {} (бросок: кол-во {}, цена {})",
            resource.name(),
            qty_roll,
            price,
            qty_roll,
            price_roll
        );
    }
}

/// Сбор ресурсов с мастерских.
/// Только мастерские с фигуркой (is_miple) производят ресурсы.
/// Количество активируемых мастерских ограничено шкалой people_workshop_map.
/// Едальня: 1 еда → 5 монет
/// Кузня: 1 металл → 1 оружие
/// Восковая: бесплатно даёт воск (если есть в хранилище)
/// Кожевенная: бесплатно даёт пушнину (если есть в хранилище)
/// Церковь: 1 воск → +1 репутация
/// Школа: 1 дерево + 1 еда → +1 человек
pub fn month_action_workshop(game: &mut Game) {
    info!("Выполняется действие месяца: сбор ресурсов с мастерских");

    for player_hash in game.get_players_hashes() {
        let player = game.get_player_by_hash(player_hash);
        let people = player.people;
        let max_workshops = *game.indexes.people_workshop_map.get(&people).unwrap_or(&1) as usize;

        let player_places = game.main_desk.get_player_places(player_hash);

        let mut activated = 0usize;

        // Собираем только мастерские с фигуркой (is_miple == true)
        let workshop_infos: Vec<(u8, WorkshopTypeEnum)> = player_places
            .iter()
            .filter(|place| place.is_miple)
            .filter_map(|place| {
                place
                    .workshop_type
                    .as_ref()
                    .map(|wt| (place.id, wt.clone()))
            })
            .collect();

        // Обрабатываем едальни первыми (они генерируют монеты)
        for (place_id, workshop_type) in &workshop_infos {
            if activated >= max_workshops {
                break;
            }

            if *workshop_type == WorkshopTypeEnum::Feeds {
                let player = game.get_player_by_hash(player_hash);
                if player.food >= 1 {
                    game.take_cube_resource(player_hash, CubeResourceTypeEnum::Food, 1);
                    game.give_money(player_hash, 5);
                    activated += 1;
                    info!(
                        "Мастерская Едальня (место {}) игрока {} произвела 5 монет",
                        place_id,
                        game.get_player_by_hash(player_hash).name
                    );
                }
            }
        }

        // Кузня: металл → оружие
        for (place_id, workshop_type) in &workshop_infos {
            if activated >= max_workshops {
                break;
            }

            if *workshop_type == WorkshopTypeEnum::Weapons {
                let player = game.get_player_by_hash(player_hash);
                if player.metal >= 1 && game.store.weapon > 0 {
                    game.take_cube_resource(player_hash, CubeResourceTypeEnum::Metal, 1);
                    game.give_cube_resource(player_hash, CubeResourceTypeEnum::Weapon, 1);
                    activated += 1;
                    info!(
                        "Мастерская Кузня (место {}) игрока {} произвела 1 оружие",
                        place_id,
                        game.get_player_by_hash(player_hash).name
                    );
                }
            }
        }

        // Церковь: воск → репутация
        for (place_id, workshop_type) in &workshop_infos {
            if activated >= max_workshops {
                break;
            }

            if *workshop_type == WorkshopTypeEnum::Church {
                let player = game.get_player_by_hash(player_hash);
                if player.wax >= 1 {
                    game.take_cube_resource(player_hash, CubeResourceTypeEnum::Wax, 1);
                    game.give_reputation(player_hash, 1);
                    activated += 1;
                    info!(
                        "Мастерская Церковь (место {}) игрока {} дала 1 репутацию",
                        place_id,
                        game.get_player_by_hash(player_hash).name
                    );
                }
            }
        }

        // Школа: дерево + еда → человек
        for (place_id, workshop_type) in &workshop_infos {
            if activated >= max_workshops {
                break;
            }

            if *workshop_type == WorkshopTypeEnum::School {
                let player = game.get_player_by_hash(player_hash);
                if player.wood >= 1 && player.food >= 1 && game.store.people > 0 {
                    game.take_cube_resource(player_hash, CubeResourceTypeEnum::Wood, 1);
                    game.take_cube_resource(player_hash, CubeResourceTypeEnum::Food, 1);
                    game.give_people(player_hash, 1);
                    activated += 1;
                    info!(
                        "Мастерская Школа (место {}) игрока {} дала 1 человека",
                        place_id,
                        game.get_player_by_hash(player_hash).name
                    );
                }
            }
        }

        // Восковая: бесплатно даёт воск если есть в хранилище
        for (place_id, workshop_type) in &workshop_infos {
            if activated >= max_workshops {
                break;
            }

            if *workshop_type == WorkshopTypeEnum::Waxes {
                if game.store.wax > 0 {
                    game.give_cube_resource(player_hash, CubeResourceTypeEnum::Wax, 1);
                    activated += 1;
                    info!(
                        "Мастерская Восковая (место {}) игрока {} произвела 1 воск",
                        place_id,
                        game.get_player_by_hash(player_hash).name
                    );
                }
            }
        }

        // Кожевенная: бесплатно даёт пушнину если есть в хранилище
        for (place_id, workshop_type) in &workshop_infos {
            if activated >= max_workshops {
                break;
            }

            if *workshop_type == WorkshopTypeEnum::Wools {
                if game.store.wool > 0 {
                    game.give_cube_resource(player_hash, CubeResourceTypeEnum::Wool, 1);
                    activated += 1;
                    info!(
                        "Мастерская Кожевенная (место {}) игрока {} произвела 1 пушнину",
                        place_id,
                        game.get_player_by_hash(player_hash).name
                    );
                }
            }
        }
    }
}

/// Люди: каждый игрок получает фигурки в зависимости от репутации
pub fn month_action_people(game: &mut Game) {
    info!("Выполняется действие месяца: люди");

    for player_hash in game.get_players_hashes() {
        let player = game.get_player_by_hash(player_hash);
        let people_gain = *game
            .indexes
            .reputation_people_map
            .get(&player.reputation)
            .unwrap_or(&0);
        if people_gain > 0 {
            let available = game.store.people;
            let give_qty = people_gain.min(available);
            if give_qty > 0 {
                game.give_people(player_hash, give_qty);
            }
        }
    }
}
