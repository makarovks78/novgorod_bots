use crate::data::future::Future;
use crate::data::month::Month;
use crate::data::turn::{TurnEnum, make_turn};
use crate::logger::init_logger;
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
    play_game(vec!["Misha".to_string(), "Vadim".to_string()], 24)?;
    Ok(())
}

pub fn play_game(player_names: Vec<String>, months: u8) -> Result<()> {
    info!("Инициализация игры...");
    let mut game = Game::new(player_names)?;
    let mut strategy = Strategy::new(&game.players)?;
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
        make_players_turn(
            &mut game,
            &turn_combinations,
            &mut strategy,
            &mut turn_stats,
        );
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
        let mut best_score = 0;
        let mut best_turns: Vec<TurnEnum> = vec![];
        for turns in turn_combinations {
            let mut player_futures = Future::new();
            let mut test_game = game.clone();
            if make_turn(
                &mut test_game,
                turns[0].clone(),
                player_hash,
                &mut player_futures,
            ) && make_turn(
                &mut test_game,
                turns[1].clone(),
                player_hash,
                &mut player_futures,
            ) && make_turn(
                &mut test_game,
                turns[2].clone(),
                player_hash,
                &mut player_futures,
            ) {
                let score = strategy.calculate_strategy_score(
                    player_hash,
                    &snapshot_diff,
                    &player_needs,
                    &player_futures,
                );

                if score > best_score {
                    best_score = score;
                    best_snapshot = snapshot_diff.clone();
                    best_turns = turns.clone();
                }
            }

            // if best_score > 0 {
            //     info!(
            //         "Лучшая комбинация ходов для игрока {}: {:?} c очками {}",
            //         player_name, best_turns, best_score
            //     );
            //     let mut snapshot = PlayerSnapshot::new(
            //         self.get_player_by_hash(player_hash),
            //         &self.main_desk,
            //         &self.field_desk,
            //     );
            //     let mut needs = PlayerSnapshot::new_empty();
            //     let mut futures = PlayerSnapshot::new_empty();
            //     make_turn(
            //         self,
            //         best_turns[0].clone(),
            //         player_hash,
            //         &mut snapshot,
            //         &mut needs,
            //         &mut futures,
            //     );
            //     *self
            //         .turn_stats
            //         .turn_stats
            //         .entry(best_turns[0].to_string())
            //         .or_insert(0) += 1;
            //     make_turn(
            //         self,
            //         best_turns[1].clone(),
            //         player_hash,
            //         &mut snapshot,
            //         &mut needs,
            //         &mut futures,
            //     );
            //     *self
            //         .turn_stats
            //         .turn_stats
            //         .entry(best_turns[1].to_string())
            //         .or_insert(0) += 1;
            //     make_turn(
            //         self,
            //         best_turns[2].clone(),
            //         player_hash,
            //         &mut snapshot,
            //         &mut needs,
            //         &mut futures,
            //     );
            //     *self
            //         .turn_stats
            //         .turn_stats
            //         .entry(best_turns[2].to_string())
            //         .or_insert(0) += 1;
            // } else {
            //     info!("Ошибка! Ни один ход не подошёл игроку {}", player_name);
            // }
        }
    }
}
