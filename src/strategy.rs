use anyhow::{Context, Result};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::hash::Hash;

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::{f64, fs, u8};

use crate::data::future::Future;
use crate::game::player::Player;
use crate::game::Game;

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
    pub needs_weight: f64,
    pub vp_weight: f64,
    pub future_weight: f64,

    pub wood_exponent: f64,
    pub food_exponent: f64,
    pub metal_exponent: f64,
    pub weapon_exponent: f64,
    pub wax_exponent: f64,
    pub wool_exponent: f64,
    pub money_exponent: f64,
    pub people_exponent: f64,
    pub reputation_exponent: f64,
    pub action_cards_exponent: f64,
    pub law_cards_exponent: f64,
    pub war_cards_exponent: f64,

    pub wood_weight: f64,
    pub food_weight: f64,
    pub metal_weight: f64,
    pub weapon_weight: f64,
    pub wax_weight: f64,
    pub wool_weight: f64,
    pub money_weight: f64,
    pub people_weight: f64,
    pub reputation_weight: f64,
    pub action_cards_weight: f64,
    pub law_cards_weight: f64,
    pub war_cards_weight: f64,

    pub greed_exponent: f64,
    pub discount_exponent: f64,
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

    pub fn evaluate_move(&self, player_hash: u64, game: &Game, futures: &Future, vp: f64) -> f64 {
        let (score, _) = self.evaluate_move_detailed(player_hash, game, futures, vp);
        score
    }

    /// Оценка хода с детальной разбивкой по компонентам.
    /// Возвращает (итоговый_скор, строка_с_разбивкой).
    pub fn evaluate_move_detailed(
        &self,
        player_hash: u64,
        game: &Game,
        futures: &Future,
        vp: f64,
    ) -> (f64, String) {
        let strategy = self.bots.get(&player_hash).unwrap();
        let w = self.strategy_weights.get(strategy).unwrap();
        let player = game.get_player_by_hash(player_hash);
        let needs = &player.needs.last;

        // =========================
        // 1. NEEDS (экспоненциальные)
        // =========================
        let needs_score = Self::critical_coverage(player.food, needs.food, w.food_exponent)
            * w.food_weight
            + Self::critical_coverage(player.wood, needs.wood, w.wood_exponent) * w.wood_weight
            + Self::critical_coverage(player.metal, needs.metal, w.metal_exponent) * w.metal_weight
            + Self::critical_coverage(player.weapon, needs.weapon, w.weapon_exponent)
                * w.weapon_weight
            + Self::critical_coverage(player.wax, needs.wax, w.wax_exponent) * w.wax_weight
            + Self::critical_coverage(player.wool, needs.wool, w.wool_exponent) * w.wool_weight
            + Self::critical_coverage(player.money.min(255) as u8, needs.money, w.money_exponent)
                * w.money_weight
            + Self::critical_coverage(player.people, needs.people, w.people_exponent)
                * w.people_weight
            + Self::critical_coverage(player.reputation, needs.reputation, w.reputation_exponent)
                * w.reputation_weight
            + Self::critical_coverage(
                player.action_cards.len().min(255) as u8,
                needs.action_cards,
                w.action_cards_exponent,
            ) * w.action_cards_weight
            + Self::critical_coverage(
                player.law_cards.len().min(255) as u8,
                needs.law_cards,
                w.law_cards_exponent,
            ) * w.law_cards_weight
            + Self::critical_coverage(
                (player.war_cards_infantry.len()
                    + player.war_cards_cavalry.len()
                    + player.war_cards_archer.len())
                .min(255) as u8,
                needs.war_cards,
                w.war_cards_exponent,
            ) * w.war_cards_weight;

        // =========================
        // 2. LONG TERM FUTURE (discount factor)
        // =========================
        let discount = w.discount_exponent.clamp(0.0, 0.99);
        let horizon_multiplier = if discount > 0.0 {
            1.0 / (1.0 - discount)
        } else {
            1.0
        };

        let long_term_future = (futures.money as f64
            + futures.wood as f64
            + futures.food as f64
            + futures.metal as f64
            + futures.weapon as f64
            + futures.wax as f64
            + futures.wool as f64)
            * horizon_multiplier;

        // =========================
        // 3. FUTURE с diminishing returns
        // =========================
        let future_value = Self::diminishing_return(player.money as f64, futures.money as f64)
            + Self::diminishing_return(player.wood as f64, futures.wood as f64)
            + Self::diminishing_return(player.food as f64, futures.food as f64)
            + Self::diminishing_return(player.metal as f64, futures.metal as f64)
            + Self::diminishing_return(player.weapon as f64, futures.weapon as f64)
            + Self::diminishing_return(player.wax as f64, futures.wax as f64)
            + Self::diminishing_return(player.wool as f64, futures.wool as f64);

        // =========================
        // 4. FINAL SCORE
        // =========================
        let comp_needs = needs_score * w.needs_weight;
        let comp_vp = vp * w.vp_weight;
        let comp_future = (future_value + long_term_future) * w.future_weight;
        let total = comp_needs + comp_vp + comp_future;

        let debug = format!(
            "needs={:.2}(raw {:.2}*w{:.2}) vp={:.2}(raw {:.1}*w{:.2}) future={:.2}(dimret {:.2}+longterm {:.2})*w{:.2} => {:.2}",
            comp_needs, needs_score, w.needs_weight,
            comp_vp, vp, w.vp_weight,
            comp_future, future_value, long_term_future, w.future_weight,
            total
        );

        (total, debug)
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
