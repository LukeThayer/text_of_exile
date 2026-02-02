# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

tui_arpg is a Rust-based Terminal User Interface Action Role Playing Game. This repo serves as a **simple demonstration and testing ground** for the [Obelisk](https://github.com/LukeThayer/Obelisk) crate—a Rust toolkit for ARPG mechanics.

### Purpose

This TUI exposes all necessary config files to configure the three crates inside Obelisk:
- **tables_core** - Weighted drop tables for loot generation
- **loot_core** - Procedural item generation, affixes, and crafting currencies
- **stat_core** - Character stats, damage calculation, and skill definitions

### Gameplay

- Turn-based combat: Player presses Space to attack an enemy
- Skills: 4 configurable skills (keys 1-4) defined in config files
- Loot: On enemy death, `tables_core` rolls drops, `loot_core` generates items
- Inventory: Items can be equipped (modifies player stats) or crafted on (apply currencies)
- Crafting is inline within the inventory panel

### Obelisk Integration

Obelisk exposes a Config loading API that takes a path:
```rust
let config = Config::load_from_dir("config/")?;
let generator = Generator::new(&config);
```

If there are issues with the Obelisk crates, report them—this repo is maintained alongside Obelisk for testing.

## Development Environment

**This is a Nix environment.** All development should be done within the Nix shell.

This project uses Nix Flakes for reproducible development. Enter the development shell before running commands:

```bash
nix develop
```

This provides: Rust stable toolchain, rust-analyzer, cargo-watch, cargo-edit, and git.

## Common Commands

```bash
cargo build          # Build the project
cargo run            # Run the application
cargo test           # Run all tests
cargo test <name>    # Run a specific test
cargo clippy         # Run lints
cargo fmt            # Format code
cargo check          # Quick compilation check
cargo watch -x run   # Auto-rebuild and run on file changes
```

## Architecture

### UI Layout (ratatui)

```
┌─────────────────┬──────────────────────┬─────────────────┐
│  PLAYER STATS   │       COMBAT         │    INVENTORY    │
│                 │                      │                 │
│  HP, stats,     │   Enemy display      │  Items list     │
│  equipment      │   Combat log         │  Equipment      │
│  summary        │   Turn indicator     │  Crafting       │
├─────────────────┴──────────────────────┴─────────────────┤
│ [SPACE] Attack  [1-4] Skills  [E] Equip  [C] Craft  [Q]  │
└──────────────────────────────────────────────────────────┘
```

### Module Structure

```
src/
├── main.rs              # Entry point, event loop
├── app.rs               # App state machine
├── config.rs            # Load all three Obelisk crate configs
├── ui/
│   ├── mod.rs
│   ├── player_panel.rs  # Left panel (stats display)
│   ├── combat_panel.rs  # Center panel (enemy + combat log)
│   └── inventory_panel.rs # Right panel (items + equip + craft)
├── game/
│   ├── mod.rs
│   ├── player.rs        # Player struct wrapping StatBlock
│   ├── enemy.rs         # Enemy struct with drop table ref
│   ├── combat.rs        # Damage resolution using stat_core
│   └── inventory.rs     # Item storage + equip/craft logic
└── input.rs             # Keybind handling
```

### Config Files

```
config/
├── items/
│   ├── bases.ron        # Item base types (loot_core)
│   └── affixes.ron      # Affix definitions (loot_core)
├── tables/
│   └── enemy.ron        # Drop table for enemy (tables_core)
├── skills/
│   └── skills.ron       # 4 skill definitions (stat_core)
├── currencies/
│   └── currencies.ron   # Crafting currencies (loot_core)
└── enemies/
    └── enemies.ron      # Enemy stat template (stat_core)
```

### Crate Demonstration Matrix

| Crate | Demonstrated By |
|-------|-----------------|
| stat_core | Player/enemy StatBlocks, damage calc, skill usage |
| tables_core | Enemy death → weighted roll → drop determination |
| loot_core | Item generation with affixes, currency crafting |
