use std::{collections::HashMap, fs};
use anyhow::{Context, Result}; 
use serde_derive::{Deserialize, Serialize};

const TURN_STATS_FILE: &str = "logs/stats/turn_stats.toml";

#[derive(Serialize, Deserialize)]
pub struct TurnStats {
    pub turn_stats: HashMap<String, u16>,
}

impl TurnStats {
    pub fn read_stats() -> Result<TurnStats> {
        let content = fs::read_to_string(TURN_STATS_FILE)
            .context(format!("Не удалось прочитать файл {}", TURN_STATS_FILE))?;

        let stats: TurnStats = toml::from_str(&content).context("Не удалось распарсить TOML")?;

        Ok(stats)
    }

    pub fn write_stats(&self) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Не удалось сериализовать в TOML")?;

        fs::write(TURN_STATS_FILE, content)
            .context(format!("Не удалось записать в файл {}", TURN_STATS_FILE))?;

        Ok(())
    }
}
