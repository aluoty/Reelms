# Reelms

A fishing RPG built with Rust and Bevy 2D.

## Game Systems

- **Cast → catch minigame → inventory**: Fish are stored, not auto-sold.
- **Economy**: Sell at shop price, or use timed **trade offers** with better multipliers.
- **Progression**: Stamina per cast, **rest** cooldown, rod/bait upgrades, luck/value buffs.
- **World**: Locations unlock by catch count and total value; weather and moon phase affect rolls.
- **Social**: Three traders (Marina, Broker Finn, Tinker Kai) with relationship-based shop discounts and trade bonuses.
- **Characters**: Alex, Tommy, Ryan archetypes with different starting stats and modifiers.
- **Save/load**: `save/progress.json`

## Project Structure

```
src/
  main.rs           — Bevy app, game states
  world.rs          — GameWorld resource (all player/world state)
  fish_engine.rs    — Fish generation
  catch_minigame.rs — Green/yellow/red pull minigame
  ui.rs             — HUD, shop, locations, character select
  save.rs           — JSON save/load
  archetype.rs      — Player archetypes
  trader.rs         — Trader NPCs
  rest.rs           — Rest cooldown manager
shell.nix           — Nix shell with Bevy Linux dependencies
```

## Requirements

- Rust 1.85+ (edition 2021)
- Linux graphics stack (Vulkan or compatible)
- For Nix users: `nix-shell` provides all Bevy system libraries

## Build & Run

### With Nix (recommended on NixOS / nix environments)

```bash
nix-shell --run "cargo run"
```

### Without Nix

Install Bevy Linux dependencies (see [Bevy docs](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)), then:

```bash
cargo run
```

Release build:

```bash
cargo run --release
```

## Gameplay

1. **Cast Line** — waits for cast time, then opens the pull minigame (SPACE or Pull Now in green zone).
2. **Sell Inventory** — sell all caught fish at base shop price.
3. **Trade Offer** — sell matching-rarity fish at a timed multiplier; generate offers via **New Trade Offer**.
4. **Shop** — rod/bait upgrades, buffs, stamina potion, special reel mod (needs Tinker Kai relationship).
5. **Locations** — Pond → River → Ocean → VolcanicBay (unlock by milestones).
6. **Rest** — +25 stamina with a cooldown.
7. **Save** — writes progress to `save/progress.json`.
