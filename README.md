# Reelms

A low-poly stylized fishing RPG built with Rust and Bevy 2D.

## Game Systems

- **Cast → catch minigame → inventory**: Fish are stored, not auto-sold.
- **Economy**: Sell at shop price, or use timed **trade offers** with better multipliers.
- **Progression**: Stamina per cast, **rest** cooldown, rod/bait upgrades, luck/value buffs.
- **World**: Locations unlock by catch count and total value; weather and moon phase affect rolls.
- **Social**: Three traders (Marina, Broker Finn, Tinker Kai) with relationship-based shop discounts and trade bonuses.
- **Characters**: Alex, Tommy, Ryan archetypes with different starting stats and modifiers.
- **Save/load**: `save/progress.json`

## Visual Style

Flat-shaded **low-poly 2D** scenery and fish models — triangular meshes, saturated palette, location-themed backgrounds (Pond, River, Misty Marsh, Ocean, Volcanic Bay).

## Project Structure

```
src/
  main.rs           — Bevy app, game states
  world.rs          — GameWorld resource (all player/world state)
  fish_engine.rs    — Fish generation (20+ species)
  models.rs         — Low-poly fish models, palette, location themes
  scene.rs          — Procedural scenery backdrop
  catch_minigame.rs — Pull minigame with low-poly fish/rod meshes
  ui.rs             — HUD, shop, locations, character select
  save.rs           — JSON save/load
  archetype.rs      — Player archetypes
  trader.rs         — Trader NPCs
  rest.rs           — Rest cooldown manager
flake.nix           — Nix flake (dev shell, Rust toolchain, Bevy deps)
```

## Requirements

- Rust 1.85+ (edition 2021)
- Linux graphics stack (Vulkan or compatible)
- For Nix users: `nix develop` provides Rust + all Bevy system libraries

## Build & Run

### With Nix (recommended on NixOS / nix environments)

```bash
nix develop --command cargo run
# or
nix run
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
5. **Locations** — Pond → River → Misty Marsh → Ocean → Volcanic Bay (unlock by milestones).
6. **Rest** — +25 stamina with a cooldown.
7. **Save** — writes progress to `save/progress.json`.
