use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::archetype::PlayerArchetype;
use crate::fish::Fish;
use crate::world::GameWorld;

pub const SAVE_PATH: &str = "save/progress.json";

#[derive(Serialize, Deserialize)]
struct SaveData {
    fish_caught: i32,
    total_catch_value: i32,
    gold: i32,
    lifetime_gold_earned: i32,
    stamina: i32,
    rod_level: i32,
    bait_level: i32,
    player_character: String,
    current_location: String,
    weather: String,
    moon_phase: String,
    luck_buff_casts: i32,
    value_buff_casts: i32,
    special_rod_power: i32,
    sounds_enabled: i32,
    relation_marina: i32,
    relation_broker_finn: i32,
    relation_tinker_kai: i32,
    next_rest_at_ms: f64,
    last_ten_values: Vec<i32>,
    inventory: Vec<Fish>,
    least_value_fish: Option<Fish>,
    most_value_fish: Option<Fish>,
}

pub fn load_progress(world: &mut GameWorld) -> bool {
    if !Path::new(SAVE_PATH).exists() {
        return false;
    }

    let json = match fs::read_to_string(SAVE_PATH) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let data: SaveData = match serde_json::from_str(&json) {
        Ok(d) => d,
        Err(_) => return false,
    };

    world.fish_caught = data.fish_caught;
    world.total_catch_value = data.total_catch_value;
    world.gold = data.gold;
    world.lifetime_gold_earned = data.lifetime_gold_earned;
    world.stamina = data.stamina;
    world.rod_level = data.rod_level;
    world.bait_level = data.bait_level;
    world.player_character = PlayerArchetype::from_name(&data.player_character);
    world.current_location = data.current_location;
    world.weather = data.weather;
    world.moon_phase = data.moon_phase;
    world.luck_buff_casts = data.luck_buff_casts;
    world.value_buff_casts = data.value_buff_casts;
    world.special_rod_power = data.special_rod_power;
    world.sounds_enabled = data.sounds_enabled == 1;
    world.relation_marina = data.relation_marina;
    world.relation_broker_finn = data.relation_broker_finn;
    world.relation_tinker_kai = data.relation_tinker_kai;
    world.rest_manager
        .set_next_rest_at_ms(data.next_rest_at_ms as i64);
    world.last_ten_values = data.last_ten_values;
    world.inventory = data.inventory;
    world.least_value_fish = data.least_value_fish;
    world.most_value_fish = data.most_value_fish;
    world.trade_offer = None;
    world.needs_character_select = false;
    true
}

pub fn save_progress(world: &GameWorld) -> bool {
    if let Some(parent) = Path::new(SAVE_PATH).parent() {
        if fs::create_dir_all(parent).is_err() {
            return false;
        }
    }

    let data = SaveData {
        fish_caught: world.fish_caught,
        total_catch_value: world.total_catch_value,
        gold: world.gold,
        lifetime_gold_earned: world.lifetime_gold_earned,
        stamina: world.stamina,
        rod_level: world.rod_level,
        bait_level: world.bait_level,
        player_character: world.player_character.display_name.to_string(),
        current_location: world.current_location.clone(),
        weather: world.weather.clone(),
        moon_phase: world.moon_phase.clone(),
        luck_buff_casts: world.luck_buff_casts,
        value_buff_casts: world.value_buff_casts,
        special_rod_power: world.special_rod_power,
        sounds_enabled: if world.sounds_enabled { 1 } else { 0 },
        relation_marina: world.relation_marina,
        relation_broker_finn: world.relation_broker_finn,
        relation_tinker_kai: world.relation_tinker_kai,
        next_rest_at_ms: world.rest_manager.next_rest_at_ms() as f64,
        last_ten_values: world.last_ten_values.clone(),
        inventory: world.inventory.clone(),
        least_value_fish: world.least_value_fish.clone(),
        most_value_fish: world.most_value_fish.clone(),
    };

    match serde_json::to_string_pretty(&data) {
        Ok(json) => fs::write(SAVE_PATH, json).is_ok(),
        Err(_) => false,
    }
}
