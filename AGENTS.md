# AGENTS.md

## Project Overview

A Rust simulation of the board game "Velikiy Novgorod" (Великий Новгород). Bots play
against each other using configurable strategies (Normal, Rich, Builder, Strategic, Risk).
Game state is loaded from TOML config files in `config/`. Logs and stats go to `logs/`.

Rust edition: **2024** (nightly features may be needed). Toolchain: rustc 1.89+.

## Build / Run / Test Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run the simulation
cargo run

# Run with release optimizations
cargo run --release

# Check for compilation errors without building
cargo check

# Run all tests (none exist yet, but this is the command)
cargo test

# Run a single test by name
cargo test test_name_here

# Run tests in a specific module
cargo test module::submodule::test_name

# Run tests with output visible
cargo test -- --nocapture

# Lint with clippy
cargo clippy

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

## Project Structure

```
src/
  main.rs             # Entry point, game loop, play_game(), make_players_turn()
  game.rs             # Game struct, resource give/take methods, sub-module declarations
  game/
    player.rs         # Player struct, PlayerDefaults (loaded from TOML)
    main_desk.rs      # City board: workshops, places, regions, leader positions
    fields_desk.rs    # Field board: land tiles (wood/food production)
    store.rs          # Shared resource pool (store)
    indexes.rs        # Lookup tables for people/reputation -> resources/VP
  data/
    mod.rs            # Sub-module declarations for data types
    turn.rs           # TurnEnum, TurnTypeEnum, make_turn(), turn combination generator
    month.rs          # MonthEnum, Month struct, month cycling logic
    month_actions.rs  # Free functions for each monthly action
    card.rs           # WarCard, ActionCard, LawCard structs
    resource.rs       # CubeResourceTypeEnum, CommonIndicatorEnum, HandResourceTypeEnum
    workshop.rs       # WorkshopTypeEnum, InputResources
    price.rs          # Price, Reward, StorePrices structs
    vp.rs             # VictoryPointsConfig, VP calculation
    needs.rs          # Needs, NeedsResoures (tracks resource demands)
    future.rs         # Future struct (predicted future gains from moves)
  strategy.rs         # Strategy assignment, scoring, statistics persistence
  deserializer.rs     # Custom serde deserializer for HashMap<u8, u8>
  logger.rs           # fern-based logger setup (stdout + timestamped file)
  turn_stats.rs       # TurnStats: turn frequency persistence
  utils.rs            # generate_hash() utility
config/               # TOML config files for game state, prices, strategies, etc.
rules.txt             # Full game rules in Russian
answers.txt           # Q&A clarifications about game rules
```

## Code Style Guidelines

### Language & Comments

- All log messages and user-facing strings are in **Russian**.
- Code identifiers (structs, functions, variables) are in **English**.
- Comments may be in Russian or English.

### Naming Conventions

- **Structs/Enums**: `PascalCase` (e.g., `Game`, `TurnEnum`, `CubeResourceTypeEnum`)
- **Enum variants**: `PascalCase` (e.g., `TurnEnum::BuyResource`, `MonthActionEnum::War`)
- **Enum type names**: suffixed with `Enum` (e.g., `TurnTypeEnum`, `WorkshopTypeEnum`)
- **Functions/methods**: `snake_case` (e.g., `make_turn`, `calculate_final_vp`)
- **Variables**: `snake_case` (e.g., `player_hash`, `best_score`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `STORE_CONFIG`, `MAX_INDEX`)
- **Config path constants**: defined as `const &str` at module top, e.g., `const VP_CONFIG: &str = "config/vp.toml";`
- **File names**: `snake_case.rs`

### Imports

Imports follow this ordering pattern (separated by blank lines when mixed):
1. `crate::` local imports
2. External crate imports (`anyhow`, `serde_derive`, `log`, `rand`, etc.)
3. `std::` imports

Specific patterns used:
```rust
use anyhow::{Context, Result};         // anyhow for error handling
use serde_derive::{Deserialize, Serialize};  // serde via serde_derive
use log::info;                         // logging macros
use crate::game::player::Player;       // local modules via crate path
```

Note: The ordering is not strictly enforced; some files mix `crate::` and external
imports freely. Use `cargo fmt` to normalize formatting.

### Types

- Small quantities: `u8` (resources, people, reputation, VP)
- Money: `u16`
- Hashes/IDs: `u64`
- Floating-point weights/scores: `f32` for VP config, `f64` for strategy scoring
- Collections: `Vec<T>` for lists, `HashMap<K, V>` for lookups
- Optional values: `Option<T>` with `#[serde(default)]` for deserialization
- No type aliases are used; types are explicit everywhere.

### Error Handling

- **`anyhow::Result<()>`** is the standard return type for fallible functions.
- Use `.with_context(|| format!("..."))` or `.context("...")` for adding error context
  on file I/O and deserialization.
- Config loading functions return `Result<Self>` using `anyhow`.
- Some functions use `unwrap()` for lookups that are expected to always succeed
  (e.g., `HashMap::get` on known keys, `find` on player lists).
- Panics (`panic!("...")`) are used only for truly invalid states (e.g., unknown enum string).
- Functions like `set_workshop_and_owner` return `Result<(), String>` (non-anyhow) in some places.

### Structs & Derive Patterns

- Serializable config structs: `#[derive(Deserialize)]` or `#[derive(Serialize, Deserialize)]`
- Game state structs: `#[derive(Debug, Clone)]` for cloning during move simulation
- Enums for game concepts: `#[derive(Debug, Clone)]`, add `PartialEq, Eq, Hash` when needed
- Serde uses `serde_derive` crate (not `#[derive(serde::Deserialize)]`).
- Use `#[serde(default)]` on optional/defaultable fields.

### Config Loading Pattern

Every config-backed struct follows this pattern:
```rust
const CONFIG_PATH: &str = "config/something.toml";

impl MyStruct {
    fn load_config() -> Result<Self> {
        let content = std::fs::read_to_string(CONFIG_PATH)?;
        let parsed: Self = toml::from_str(&content)?;
        Ok(parsed)
    }

    pub fn new() -> Result<Self> {
        Ok(Self::load_config()?)
    }
}
```

### Logging

- Uses `log` crate macros (`info!`) with `fern` backend.
- Logger writes to both stdout and a timestamped file in `logs/`.
- Log messages are in Russian, describing game actions.
- Format: plain message text only (no timestamps/levels in output format).

### Module Organization

- Sub-modules with multiple files use a directory + `mod.rs` pattern (see `data/mod.rs`).
- Parent modules declare children with `pub mod child;` in their root file.
- `game.rs` acts as both the `Game` struct file and the parent of `game/` sub-modules.

### Testing

- No tests exist in the codebase yet. When adding tests:
  - Place unit tests in a `#[cfg(test)] mod tests { ... }` block at the bottom of each file.
  - Integration tests go in a top-level `tests/` directory.
  - Run with `cargo test` or `cargo test test_name -- --nocapture` for output.

### Formatting

- Use `cargo fmt` (rustfmt) for all formatting.
- Indent: 4 spaces.
- Line width: default rustfmt (100 chars).
- Trailing commas on multi-line struct/enum/function definitions.
