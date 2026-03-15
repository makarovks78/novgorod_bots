use anyhow::{Context, Result};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::hash::Hash;

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{f64, fs, u8};
use std::path::Path;

use crate::data::future::Future;
use crate::data::workshop::WorkshopTypeEnum;
use crate::game::Game;
use crate::game::player::Player;

const NORMAL_WEIGHT_CONFIG: &str = "config/strategy/normal.toml";
const RICH_WEIGHT_CONFIG: &str = "config/strategy/rich.toml";
const BUILDER_WEIGHT_CONFIG: &str = "config/strategy/builder.toml";
const STRATEGIC_WEIGHT_CONFIG: &str = "config/strategy/strategic.toml";
const RISK_WEIGHT_CONFIG: &str = "config/strategy/risk.toml";
const STATS_FILE: &str = "logs/stats/strategy_stats.toml";
const MAX_NEEDS_WEIGHT: i16 = 100;

#[derive(Hash, Eq, PartialEq)]
pub enum StrategyTypeEnum {
    Normal,
    Rich,
    Builder,
    Strategic,
    Risk,
}

impl StrategyTypeEnum {
    fn to_string(&self) -> String {
        match self {
            StrategyTypeEnum::Normal => "Normal".to_string(),
            StrategyTypeEnum::Rich => "Rich".to_string(),
            StrategyTypeEnum::Builder => "Builder".to_string(),
            StrategyTypeEnum::Strategic => "Strategic".to_string(),
            StrategyTypeEnum::Risk => "Risk".to_string(),
        }
    }

    fn from_string(s: &str) -> StrategyTypeEnum {
        match s {
            "Normal" => StrategyTypeEnum::Normal,
            "Rich" => StrategyTypeEnum::Rich,
            "Builder" => StrategyTypeEnum::Builder,
            "Strategic" => StrategyTypeEnum::Strategic,
            "Risk" => StrategyTypeEnum::Risk,
            _ => panic!("Invalid strategy type"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StrategyStatistics {
    games_played: u16,
    strategies: HashMap<String, u16>,
}

#[derive(Deserialize)]
pub struct StrategyWeight {
    pub needs_weight: u64,
    pub vp_weight: u64,
    pub future_weight: u64,

    pub wood_exponent: u64,
    pub food_exponent: u64,
    pub metal_exponent: u64,
    pub weapon_exponent: u64,
    pub wax_exponent: u64,
    pub wool_exponent: u64,
    pub money_exponent: u64,
    pub people_exponent: u64,
    pub reputation_exponent: u64,
    pub action_cards_exponent: u64,
    pub law_cards_exponent: u64,
    pub war_cards_exponent: u64,

    pub wood_weight: u64,
    pub food_weight: u64,
    pub metal_weight: u64,
    pub weapon_weight: u64,
    pub wax_weight: u64,
    pub wool_weight: u64,
    pub money_weight: u64,
    pub people_weight: u64,
    pub reputation_weight: u64,
    pub action_cards_weight: u64,
    pub law_cards_weight: u64,
    pub war_cards_weight: u64,

    pub greed_exponent: u64,
    pub discount_exponent: u64,
}

impl StrategyWeight {
    pub fn load_strategy_weights(file: &str) -> Result<StrategyWeight> {
        let toml_content = fs::read_to_string(Path::new(file))
            .with_context(|| format!("Failed to read strategy config: {}", file))?;
        let strategy_weights: StrategyWeight = toml::from_str(&toml_content)
            .with_context(|| format!("Failed to parse strategy config: {}", file))?;
        Ok(strategy_weights)
    }
}

pub struct Strategy {
    bots: HashMap<u64, StrategyTypeEnum>,
    strategy_weights: HashMap<StrategyTypeEnum, StrategyWeight>,
}

impl Strategy {
    pub fn new(players: &Vec<Player>) -> Result<Self> {
        let mut bots = HashMap::new();
        let mut rng: ThreadRng = rand::rng();

        for player in players {
            let strategy_type = match rng.random_range(0..5) {
                0 => StrategyTypeEnum::Normal,
                1 => StrategyTypeEnum::Rich,
                2 => StrategyTypeEnum::Builder,
                3 => StrategyTypeEnum::Risk,
                4 => StrategyTypeEnum::Strategic,
                _ => StrategyTypeEnum::Normal,
            };
            bots.insert(player.hash, strategy_type);
        }

        let normal_weights = StrategyWeight::load_strategy_weights(NORMAL_WEIGHT_CONFIG)?;
        let rich_weights = StrategyWeight::load_strategy_weights(RICH_WEIGHT_CONFIG)?;
        let builder_weights = StrategyWeight::load_strategy_weights(BUILDER_WEIGHT_CONFIG)?;
        let risk_weights = StrategyWeight::load_strategy_weights(RISK_WEIGHT_CONFIG)?;
        let startegic_weights = StrategyWeight::load_strategy_weights(STRATEGIC_WEIGHT_CONFIG)?;

        let mut strategy_weights = HashMap::new();
        strategy_weights.insert(StrategyTypeEnum::Normal, normal_weights);
        strategy_weights.insert(StrategyTypeEnum::Rich, rich_weights);
        strategy_weights.insert(StrategyTypeEnum::Builder, builder_weights);
        strategy_weights.insert(StrategyTypeEnum::Risk, risk_weights);
        strategy_weights.insert(StrategyTypeEnum::Strategic, startegic_weights);

        Ok(Strategy {
            bots,
            strategy_weights,
        })
    }

    pub fn calculate_strategy_score(
        &self,
        player_hash: u64,
        game: &mut Game,
        futures: &Future,
    ) -> i16 {
        let strategy = self.bots.get(&player_hash).unwrap();
        let weights = self.strategy_weights.get(strategy).unwrap();
        let player = game.get_player_by_hash(player_hash);
        let score = 0;
        score
    }

    pub fn evaluate_move(
        &self,
        player_hash: u64,
        game: &mut Game,
        futures: &Future,
        vp: f64,
    ) -> f64 {
        let strategy = self.bots.get(&player_hash).unwrap();
        let weights = self.strategy_weights.get(strategy).unwrap();
        let player = game.get_player_by_hash(player_hash);
        let score: u64 = 0.0;

        // =========================
        // 1. NEEDS (экспоненциальные)
        // =========================

        let food_score = self.critical_coverage(player.food, player.needs.last.food, self.strategy_weights.get(strategy).unwrap().food_exponent);

        let wood_score =
            critical_coverage(player_after.wood, needs.wood, personality.wood_exponent);

        let money_score =
            critical_coverage(player_after.money, needs.money, personality.money_exponent);

        let needs_score = food_score * personality.food_weight
            + wood_score * personality.wood_weight
            + money_score * personality.money_weight;

        // =========================
        // 2. LONG TERM FUTURE (discount factor)
        // =========================

        let horizon_multiplier = 1.0 / (1.0 - personality.discount);

        let long_term_future =
            (future.money as f64 + future.wood as f64 + future.food as f64) * horizon_multiplier;

        // =========================
        // 3. FUTURE с diminishing returns
        // =========================

        let future_value = diminishing_return(player_after.money as f64, future.money as f64)
            + diminishing_return(player_after.wood as f64, future.wood as f64)
            + diminishing_return(player_after.food as f64, future.food as f64);

        // =========================
        // 4. GREED (накопление)
        // =========================

        let stockpile =
            player_after.money as f64 + player_after.wood as f64 + player_after.food as f64;

        let stockpile_value = stockpile.powf(1.0 + personality.greed * 0.3);

        // =========================
        // 5. FINAL SCORE
        // =========================

        let score = needs_score * personality.needs_weight
            + vp * personality.vp_weight
            + (future_value + long_term_future) * personality.future_weight
            + stockpile_value * personality.greed;

        score
    }

    pub fn to_string(&self, players: &Vec<Player>) -> String {
        let mut result = String::from("Назначенные стратегии:\n");
        for player in players {
            if let Some(strategy) = self.bots.get(&player.hash) {
                let strategy_name = match strategy {
                    StrategyTypeEnum::Normal => "Сбалансированная",
                    StrategyTypeEnum::Rich => "Жадный",
                    StrategyTypeEnum::Builder => "Строитель",
                    StrategyTypeEnum::Strategic => "Стратег",
                    StrategyTypeEnum::Risk => "Рисковый",
                };
                result.push_str(&format!("{}: {}\n", player.name, strategy_name));
            }
        }
        result
    }

    fn critical_coverage(current: u8, need: u8, exponent: f64) -> f64 {
        if need == 0 {
            return 1.0;
        }

        if current >= need {
            return 1.0;
        }

        let deficit_ratio = (need - current) as f64 / need as f64;

        1.0 - deficit_ratio.powf(exponent)
    }

    fn diminishing_return(current: f64, gain: f64) -> f64 {
        if current <= 0.0 {
            return gain;
        }

        (current + gain).ln() - current.ln()
    }

    fn needs_score(&self, needs: i16, snapshot: i16) -> i16 {
        if needs > 0 {
            let score = snapshot * MAX_NEEDS_WEIGHT / needs;
            if score > MAX_NEEDS_WEIGHT {
                MAX_NEEDS_WEIGHT
            } else {
                score
            }
        } else {
            0
        }
    }

    pub fn update_stats(&self, game: &Game) -> Result<()> {
        let mut stats = self.read_stats().unwrap_or_else(|_| StrategyStatistics {
            games_played: 0,
            strategies: vec![
                ((StrategyTypeEnum::Normal).to_string(), 0),
                ((StrategyTypeEnum::Builder).to_string(), 0),
                ((StrategyTypeEnum::Rich).to_string(), 0),
            ]
            .into_iter()
            .collect(),
        });
        stats.games_played += 1;

        let mut sorted_players: Vec<&Player> = game.players.iter().collect();
        sorted_players.sort_by(|a, b| b.vp.cmp(&a.vp));

        let ranking: HashMap<u64, u8> = sorted_players
            .iter()
            .enumerate()
            .map(|(index, player)| (player.hash, index as u8 + 1))
            .collect();

        for (player_hash, rank) in ranking {
            for (bot_hash, strategy) in self.bots.iter() {
                if bot_hash == &player_hash {
                    let rank_precent = match rank {
                        1 => 100,
                        2 => 66,
                        3 => 33,
                        _ => 0,
                    };
                    let old_rank = stats.strategies.get(&strategy.to_string()).unwrap().clone();
                    stats
                        .strategies
                        .insert(strategy.to_string(), ((old_rank + rank_precent) / 2) as u16);
                }
            }
        }

        self.write_stats(&stats)?;

        Ok(())
    }

    fn read_stats(&self) -> Result<StrategyStatistics> {
        let content = fs::read_to_string(STATS_FILE)
            .context(format!("Не удалось прочитать файл {}", STATS_FILE))?;

        let stats: StrategyStatistics =
            toml::from_str(&content).context("Не удалось распарсить TOML")?;

        Ok(stats)
    }

    fn write_stats(&self, stats: &StrategyStatistics) -> Result<()> {
        let content = toml::to_string_pretty(stats).context("Не удалось сериализовать в TOML")?;

        fs::write(STATS_FILE, content)
            .context(format!("Не удалось записать в файл {}", STATS_FILE))?;

        Ok(())
    }

}
