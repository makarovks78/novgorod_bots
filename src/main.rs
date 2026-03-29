use crate::data::future::Future;
use crate::data::month::Month;
use crate::data::needs::Needs;
use crate::data::turn::{make_turn, TurnEnum};
use crate::logger::{init_logger, set_silent};
use crate::{game::Game, strategy::Strategy, turn_stats::TurnStats};
use anyhow::Result;
use log::info;

mod data;
mod deserializer;
mod game;
mod logger;
mod strategy;
mod turn_stats;
mod utils;

fn main() -> Result<()> {
    init_logger()?;
    play_game(vec!["Misha".to_string(), "Vadim".to_string()], 12)?;
    Ok(())
}

pub fn play_game(player_names: Vec<String>, months: u8) -> Result<()> {
    info!("Инициализация игры...");
    let mut game = Game::new(player_names)?;
    let strategy = Strategy::new(&game.players)?;
    let mut months = Month::new(months);
    let mut turn_stats = TurnStats::read_stats()?;
    let turn_combinations = TurnEnum::generate_unique_combinations();
    info!("{}", game.to_string());
    let mut next = true;
    info!("{}", strategy.to_string(&game.players));

    while next {
        info!("========================================");
        info!("Месяц {}", months.get_current_month_name());
        months.process_current_month_actions(&mut game);
        make_players_turn(&mut game, &turn_combinations, &strategy, &mut turn_stats);
        next = months.next();
        info!("{}", game.to_string());
    }
    game.calculate_final_vp()?;
    //    strategy.update_stats(&game)?;
    //    turn_stats.write_stats()?;
    Ok(())
}

pub fn make_players_turn(
    game: &mut Game,
    turn_combinations: &Vec<Vec<TurnEnum>>,
    strategy: &Strategy,
    turn_stats: &mut TurnStats,
) {
    let player_hashes: Vec<u64> = game.get_players_hashes();
    for player_hash in player_hashes {
        let player_name = game.get_player_by_hash(player_hash).name.clone();
        let original_vp = game.get_player_by_hash(player_hash).vp;
        let mut best_score: f64 = f64::NEG_INFINITY;
        let mut best_turns: Vec<TurnEnum> = vec![];
        // Накопитель потребностей: собирает needs из всех тестовых комбинаций (max-семантика)
        let mut player_needs = Needs::new();
        set_silent(true); // Подавляем логи при прогонке тестовых ходов
        for turns in turn_combinations {
            let mut player_futures = Future::new();
            let mut test_game = game.clone();
            if make_turn(
                &mut test_game,
                turns[0].clone(),
                player_hash,
                &mut player_futures,
                &mut player_needs,
            ) && make_turn(
                &mut test_game,
                turns[1].clone(),
                player_hash,
                &mut player_futures,
                &mut player_needs,
            ) && make_turn(
                &mut test_game,
                turns[2].clone(),
                player_hash,
                &mut player_futures,
                &mut player_needs,
            ) {
                let vp_gained =
                    test_game.get_player_by_hash(player_hash).vp as f64 - original_vp as f64;
                let score =
                    strategy.evaluate_move(player_hash, &test_game, &player_futures, vp_gained);

                if score > best_score {
                    best_score = score;
                    best_turns = turns.clone();
                }
            }
        }
        set_silent(false); // Включаем логи обратно для реальных ходов

        // Снапшотим потребности: current → last, сбрасываем current
        player_needs.move_current_to_last();
        // Записываем потребности в реального игрока
        game.get_player_by_hash_mut(player_hash).needs = player_needs;

        if best_score > f64::NEG_INFINITY {
            // Детальный лог оценки лучшей комбинации (прогон на клоне — тихий режим)
            {
                set_silent(true);
                let mut debug_futures = Future::new();
                let mut debug_needs = Needs::new();
                let mut debug_game = game.clone();
                let _ = make_turn(
                    &mut debug_game,
                    best_turns[0].clone(),
                    player_hash,
                    &mut debug_futures,
                    &mut debug_needs,
                );
                let _ = make_turn(
                    &mut debug_game,
                    best_turns[1].clone(),
                    player_hash,
                    &mut debug_futures,
                    &mut debug_needs,
                );
                let _ = make_turn(
                    &mut debug_game,
                    best_turns[2].clone(),
                    player_hash,
                    &mut debug_futures,
                    &mut debug_needs,
                );
                set_silent(false);
                let debug_vp =
                    debug_game.get_player_by_hash(player_hash).vp as f64 - original_vp as f64;
                let (_, breakdown) = strategy.evaluate_move_detailed(
                    player_hash,
                    &debug_game,
                    &debug_futures,
                    debug_vp,
                );
                info!("[{}] Ходы: {:?} | {}", player_name, best_turns, breakdown);
            }
            let mut futures = Future::new();
            let mut discard_needs = Needs::new();
            make_turn(
                game,
                best_turns[0].clone(),
                player_hash,
                &mut futures,
                &mut discard_needs,
            );
            *turn_stats
                .turn_stats
                .entry(best_turns[0].to_string())
                .or_insert(0) += 1;
            make_turn(
                game,
                best_turns[1].clone(),
                player_hash,
                &mut futures,
                &mut discard_needs,
            );
            *turn_stats
                .turn_stats
                .entry(best_turns[1].to_string())
                .or_insert(0) += 1;
            make_turn(
                game,
                best_turns[2].clone(),
                player_hash,
                &mut futures,
                &mut discard_needs,
            );
            *turn_stats
                .turn_stats
                .entry(best_turns[2].to_string())
                .or_insert(0) += 1;
        } else {
            info!("Ошибка! Ни один ход не подошёл игроку {}", player_name);
        }
    }
}
