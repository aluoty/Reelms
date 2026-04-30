# Fishing Game (Java UI + C++ Engine)

This is a base starter project for a Fishing Game:

- Java (`Swing`) for the UI and game loop interaction.
- C++ for fish generation and algorithmic logic.

## Project Structure

- `java/src/com/fishinggame/Main.java` - Java UI entry point.
- `cpp/fish_engine.cpp` - C++ fish generation engine.
- `build.sh` - Builds Java classes and C++ binary.
- `run.sh` - Runs the Java game UI.

## Requirements

- JDK 17+ (or JDK 11+ with minor adjustments)
- `g++` (C++17 compatible)
- Linux/macOS shell

## Build

```bash
./build.sh
```

## Run

```bash
./run.sh
```

## Gameplay (Expanded)

- Click **Cast Line** to catch fish into your inventory (not auto-sold).
- Economy loop:
  - **Sell Inventory** at shop price (always available).
  - **Trade Offer** can pay better multipliers, but expires quickly.
- Progression:
  - **Stamina** cost per cast, recover via **Rest** or stamina potion.
  - Buy **rod** and **bait** upgrades in **Shop** for better catch outcomes.
  - Buy **buffs** (luck/value) with gold.
- World systems:
  - **Locations** unlock when fish count and total catch value thresholds are met.
  - Each location has shared fish + location-exclusive exotic fish.
  - **Weather** and **moon phase** affect rarity/value and cast time.
    - Rain shortens cast time and stamina cost.
    - Blue Moon gives strong positive rarity/value pressure.
    - Blood Moon gives negative pressure.
- Save/load:
  - Use **Save** button or **Save and Exit** prompt.
  - Progress stored at `save/progress.json`.
