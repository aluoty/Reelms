use rand::Rng;

use crate::fish::Fish;

pub fn generate_fish(
    rng: &mut impl Rng,
    fish_caught: i32,
    location: &str,
    rod_level: i32,
    bait_level: i32,
    weather: &str,
    moon: &str,
    luck_buff_casts: i32,
    value_buff_casts: i32,
) -> Fish {
    let rod_level = rod_level.max(1);
    let bait_level = bait_level.max(1);

    let (species_pool, location_exotics, weight_min, weight_max) = match location {
        "River" => (
            vec![
                "Carp", "Salmon", "Trout", "Catfish", "Pike", "Bass",
            ],
            vec!["GoldenKoi", "SilverEel"],
            0.8,
            15.0,
        ),
        "Ocean" => (
            vec!["Tuna", "Mackerel", "Swordfish", "Marlin", "Bass"],
            vec!["BluefinTitan", "Glowfin"],
            1.5,
            30.0,
        ),
        "VolcanicBay" => (
            vec!["LavaSnapper", "AshGrouper", "Tuna", "Marlin"],
            vec!["MagmaRay", "PhoenixKoi"],
            2.5,
            35.0,
        ),
        _ => (
            vec!["Carp", "Salmon", "Trout", "Catfish", "Pike"],
            vec![],
            0.4,
            11.0,
        ),
    };

    let mut rarity_shift = 0;
    rarity_shift += (fish_caught / 25).min(8);
    rarity_shift += (rod_level - 1).max(0);
    rarity_shift += (bait_level - 1).max(0);
    if luck_buff_casts > 0 {
        rarity_shift += 3;
    }
    if weather == "Storm" {
        rarity_shift += 2;
    }
    if weather == "Rain" {
        rarity_shift += 1;
    }
    if moon == "BlueMoon" {
        rarity_shift += 7;
    }
    if moon == "BloodMoon" {
        rarity_shift -= 7;
    }

    let roll = (rng.random_range(1..=100) - rarity_shift).clamp(1, 100);
    let mut rarity = rarity_from_roll(roll);

    let mut species = species_pool[rng.random_range(0..species_pool.len())].to_string();

    if !location_exotics.is_empty() {
        let mut chance = 6;
        if location == "Ocean" {
            chance = 8;
        }
        if location == "VolcanicBay" {
            chance = 12;
        }
        chance += bait_level - 1;
        if moon == "BlueMoon" {
            chance += 5;
        }
        if moon == "BloodMoon" {
            chance -= 3;
        }
        if rng.random_range(1..=100) <= chance.max(2) {
            species = location_exotics[rng.random_range(0..location_exotics.len())].to_string();
            if rarity_rank(&rarity) < rarity_rank("Epic") {
                rarity = "Epic".to_string();
            }
        }
    }

    let weight_kg: f64 = rng.random_range(weight_min..weight_max);

    let rarity_multiplier = match rarity.as_str() {
        "Uncommon" => 1.5,
        "Rare" => 2.4,
        "Epic" => 4.0,
        "Legendary" => 7.5,
        "Mythic" => 10.0,
        _ => 1.0,
    };

    let location_multiplier = match location {
        "River" => 1.2,
        "Ocean" => 1.55,
        "VolcanicBay" => 2.1,
        _ => 1.0,
    };

    let mut weather_multiplier = 1.0;
    if weather == "Storm" {
        weather_multiplier = 1.15;
    }
    if moon == "BlueMoon" {
        weather_multiplier *= 1.30;
    }
    if moon == "BloodMoon" {
        weather_multiplier *= 0.80;
    }
    if value_buff_casts > 0 {
        weather_multiplier *= 1.20;
    }

    let value = (weight_kg * 18.0 * rarity_multiplier * location_multiplier * weather_multiplier) as i32;

    Fish {
        species,
        rarity,
        weight_kg,
        value,
    }
}

fn rarity_from_roll(roll: i32) -> String {
    match roll {
        1..=40 => "Common",
        41..=67 => "Uncommon",
        68..=84 => "Rare",
        85..=95 => "Epic",
        96..=99 => "Legendary",
        _ => "Mythic",
    }
    .to_string()
}

pub fn rarity_rank(rarity: &str) -> i32 {
    match rarity {
        "Common" => 1,
        "Uncommon" => 2,
        "Rare" => 3,
        "Epic" => 4,
        "Legendary" => 5,
        "Mythic" => 6,
        _ => 1,
    }
}
