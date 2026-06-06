use std::time::{SystemTime, UNIX_EPOCH};

use bevy::prelude::Resource;
use rand::Rng;

use crate::archetype::PlayerArchetype;
use crate::fish::Fish;
use crate::fish_engine::{generate_fish, rarity_rank};
use crate::rest::RestManager;
use crate::trader::TraderNpc;

pub const MAX_STAMINA: i32 = 100;
pub const MAX_ROD_LEVEL: i32 = 8;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[derive(Clone, Debug)]
pub struct TradeOffer {
    pub trader_name: String,
    pub min_rarity: String,
    pub multiplier: f64,
    pub expires_at_ms: i64,
}

impl TradeOffer {
    pub fn is_active(&self) -> bool {
        now_ms() < self.expires_at_ms
    }

    pub fn seconds_remaining(&self) -> i64 {
        ((self.expires_at_ms - now_ms()).max(0)) / 1000
    }
}

#[derive(Resource, Clone, Debug)]
pub struct GameWorld {
    pub fish_caught: i32,
    pub total_catch_value: i32,
    pub gold: i32,
    pub lifetime_gold_earned: i32,
    pub stamina: i32,
    pub rod_level: i32,
    pub bait_level: i32,
    pub current_location: String,
    pub weather: String,
    pub moon_phase: String,
    pub casts_since_weather_update: i32,
    pub luck_buff_casts: i32,
    pub value_buff_casts: i32,
    pub special_rod_power: i32,
    pub sounds_enabled: bool,
    pub relation_marina: i32,
    pub relation_broker_finn: i32,
    pub relation_tinker_kai: i32,
    pub player_character: PlayerArchetype,
    pub last_ten_values: Vec<i32>,
    pub inventory: Vec<Fish>,
    pub least_value_fish: Option<Fish>,
    pub most_value_fish: Option<Fish>,
    pub trade_offer: Option<TradeOffer>,
    pub rest_manager: RestManager,
    pub status_message: String,
    pub log_lines: Vec<String>,
    pub needs_character_select: bool,
    pub show_guide: bool,
    pub casting: bool,
    pub cast_timer: f32,
    pub pending_fish: Option<Fish>,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            fish_caught: 0,
            total_catch_value: 0,
            gold: 0,
            lifetime_gold_earned: 0,
            stamina: MAX_STAMINA,
            rod_level: 1,
            bait_level: 1,
            current_location: "Pond".to_string(),
            weather: "Clear".to_string(),
            moon_phase: "Normal".to_string(),
            casts_since_weather_update: 0,
            luck_buff_casts: 0,
            value_buff_casts: 0,
            special_rod_power: 0,
            sounds_enabled: true,
            relation_marina: 0,
            relation_broker_finn: 0,
            relation_tinker_kai: 0,
            player_character: PlayerArchetype::ALEX,
            last_ten_values: Vec::new(),
            inventory: Vec::new(),
            least_value_fish: None,
            most_value_fish: None,
            trade_offer: None,
            rest_manager: RestManager::default(),
            status_message: "Ready to fish.".to_string(),
            log_lines: vec![
                "Catch log:".to_string(),
                "- You can sell fish at shop or wait for a timed trade offer.".to_string(),
            ],
            needs_character_select: true,
            show_guide: true,
            casting: false,
            cast_timer: 0.0,
            pending_fish: None,
        }
    }
}

impl GameWorld {
    pub fn log(&mut self, line: impl Into<String>) {
        self.log_lines.push(format!("- {}", line.into()));
        if self.log_lines.len() > 80 {
            self.log_lines.drain(0..10);
        }
    }

    pub fn apply_archetype(&mut self, archetype: PlayerArchetype) {
        self.player_character = archetype;
        self.gold = archetype.starting_gold;
        self.rod_level = archetype.starting_rod_level;
        self.bait_level = archetype.starting_bait_level;
        self.stamina = archetype.starting_stamina.min(MAX_STAMINA);
        self.status_message = format!("Character chosen: {}.", archetype.display_name);
        self.needs_character_select = false;
    }

    pub fn stamina_cost_per_cast(&self) -> i32 {
        let mut base = 8 - (self.rod_level - 1);
        if self.rod_level >= 6 {
            base -= 1;
        }
        if self.weather == "Rain" {
            base = (base - 2).max(3);
        }
        base.max(3)
    }

    pub fn cast_time_secs(&self) -> f32 {
        let archetype = self.player_character;
        let mut base = 1500 - (self.rod_level - 1) * 170;
        if self.rod_level >= 6 {
            base -= 120;
        }
        if self.weather == "Rain" {
            base -= 300;
        }
        let scaled = (base as f64 * archetype.cast_speed_multiplier).round() as i32;
        (scaled.max(450) as f32) / 1000.0
    }

    pub fn rod_skin_name(&self) -> &'static str {
        match self.rod_level {
            0 => "Worn Reed Pole",
            1 => "Bamboo Starter",
            2 => "Fiberglass Trail",
            3 => "Composite Stream",
            4 => "Graphite Pro",
            5 => "Carbon Elite",
            6 => "Titan Weave",
            7 => "Abyss Arc",
            _ => "Mythril Current",
        }
    }

    pub fn rod_catch_difficulty_reduction(&self) -> f64 {
        let base = match self.rod_level {
            1 => 0.00,
            2 => 0.08,
            3 => 0.14,
            4 => 0.20,
            5 => 0.27,
            6 => 0.33,
            7 => 0.39,
            _ => 0.46,
        };
        (base + self.special_rod_power as f64 * 0.03).min(0.65)
    }

    pub fn relationship(&self, trader: &str) -> i32 {
        match trader {
            "Marina" => self.relation_marina,
            "Broker Finn" => self.relation_broker_finn,
            "Tinker Kai" => self.relation_tinker_kai,
            _ => 0,
        }
    }

    pub fn add_relationship(&mut self, trader: &str, amount: i32) {
        let adjusted = ((amount as f64 * self.player_character.relation_gain_multiplier).round()
            as i32)
            .max(1);
        match trader {
            "Marina" => self.relation_marina = (self.relation_marina + adjusted).min(40),
            "Broker Finn" => {
                self.relation_broker_finn = (self.relation_broker_finn + adjusted).min(40)
            }
            "Tinker Kai" => self.relation_tinker_kai = (self.relation_tinker_kai + adjusted).min(40),
            _ => {}
        }
    }

    pub fn best_friend_trader(&self) -> &'static str {
        let marina = self.relation_marina;
        let finn = self.relation_broker_finn;
        let kai = self.relation_tinker_kai;
        if marina >= finn && marina >= kai {
            "Marina"
        } else if finn >= marina && finn >= kai {
            "Broker Finn"
        } else {
            "Tinker Kai"
        }
    }

    pub fn shop_discount_multiplier(&self) -> f64 {
        let relation = self.relationship(self.best_friend_trader());
        let discount = (relation as f64 * 0.0075).min(0.22);
        1.0 - discount
    }

    pub fn discounted_cost(&self, base_cost: i32) -> i32 {
        let cost = base_cost as f64
            * self.shop_discount_multiplier()
            * self.player_character.shop_cost_multiplier;
        cost.round().max(1.0) as i32
    }

    pub fn average_last_ten(&self) -> f64 {
        if self.last_ten_values.is_empty() {
            return 0.0;
        }
        let sum: i32 = self.last_ten_values.iter().sum();
        sum as f64 / self.last_ten_values.len() as f64
    }

    pub fn update_value_stats(&mut self, fish: &Fish) {
        if self.last_ten_values.len() == 10 {
            self.last_ten_values.remove(0);
        }
        self.last_ten_values.push(fish.value);

        if self.least_value_fish.as_ref().map_or(true, |f| fish.value < f.value) {
            self.least_value_fish = Some(fish.clone());
        }
        if self.most_value_fish.as_ref().map_or(true, |f| fish.value > f.value) {
            self.most_value_fish = Some(fish.clone());
        }
    }

    pub fn consume_buff_durations(&mut self) {
        if self.luck_buff_casts > 0 {
            self.luck_buff_casts -= 1;
        }
        if self.value_buff_casts > 0 {
            self.value_buff_casts -= 1;
        }
    }

    pub fn update_world_cycle(&mut self, rng: &mut impl Rng) {
        self.casts_since_weather_update = 0;
        let weather_roll: i32 = rng.random_range(0..100);
        self.weather = if weather_roll < 55 {
            "Clear".to_string()
        } else if weather_roll < 85 {
            "Rain".to_string()
        } else {
            "Storm".to_string()
        };

        let moon_roll: i32 = rng.random_range(0..100);
        self.moon_phase = if moon_roll < 8 {
            "BlueMoon".to_string()
        } else if moon_roll < 16 {
            "BloodMoon".to_string()
        } else {
            "Normal".to_string()
        };

        self.log(format!(
            "Weather changed to {}, moon is {}.",
            self.weather, self.moon_phase
        ));
    }

    pub fn is_location_unlocked(&self, location: &str) -> bool {
        match location {
            "Pond" => true,
            "River" => self.fish_caught >= 12 && self.total_catch_value >= 450,
            "MistyMarsh" => self.fish_caught >= 20 && self.total_catch_value >= 900,
            "Ocean" => self.fish_caught >= 30 && self.total_catch_value >= 1500,
            "VolcanicBay" => self.fish_caught >= 60 && self.total_catch_value >= 4200,
            _ => false,
        }
    }

    pub fn ensure_location_access(&mut self) {
        if self.current_location == "River" && !self.is_location_unlocked("River") {
            self.current_location = "Pond".to_string();
        }
        if self.current_location == "MistyMarsh" && !self.is_location_unlocked("MistyMarsh") {
            self.current_location = if self.is_location_unlocked("River") {
                "River".to_string()
            } else {
                "Pond".to_string()
            };
        }
        if self.current_location == "Ocean" && !self.is_location_unlocked("Ocean") {
            self.current_location = if self.is_location_unlocked("MistyMarsh") {
                "MistyMarsh".to_string()
            } else if self.is_location_unlocked("River") {
                "River".to_string()
            } else {
                "Pond".to_string()
            };
        }
        if self.current_location == "VolcanicBay" && !self.is_location_unlocked("VolcanicBay") {
            if self.is_location_unlocked("Ocean") {
                self.current_location = "Ocean".to_string();
            } else if self.is_location_unlocked("River") {
                self.current_location = "River".to_string();
            } else {
                self.current_location = "Pond".to_string();
            }
        }
    }

    pub fn set_location(&mut self, location: &str) -> bool {
        if !self.is_location_unlocked(location) {
            self.status_message =
                "Location still locked. Catch more fish/value to unlock.".to_string();
            return false;
        }
        self.current_location = location.to_string();
        self.status_message = format!("Travelled to {}.", location);
        true
    }

    pub fn start_cast(&mut self) -> bool {
        if self.casting {
            return false;
        }
        if self.stamina < self.stamina_cost_per_cast() {
            self.status_message = "Too tired. Rest or buy stamina potion.".to_string();
            return false;
        }
        self.stamina = (self.stamina - self.stamina_cost_per_cast()).max(0);
        self.casting = true;
        self.cast_timer = self.cast_time_secs();
        self.status_message = format!(
            "Casting... ({:.0} ms)",
            self.cast_time_secs() * 1000.0
        );
        true
    }

    pub fn tick_cast(&mut self, dt: f32, rng: &mut impl Rng) -> Option<Fish> {
        if !self.casting {
            return None;
        }
        self.cast_timer -= dt;
        if self.cast_timer > 0.0 {
            return None;
        }
        self.casting = false;
        let fish = generate_fish(
            rng,
            self.fish_caught,
            &self.current_location,
            self.rod_level.max(1),
            self.bait_level,
            &self.weather,
            &self.moon_phase,
            self.luck_buff_casts,
            self.value_buff_casts,
        );
        self.pending_fish = Some(fish.clone());
        Some(fish)
    }

    pub fn on_catch_success(&mut self, fish: Fish, rng: &mut impl Rng) {
        self.consume_buff_durations();
        self.casts_since_weather_update += 1;
        if self.casts_since_weather_update >= 4 {
            self.update_world_cycle(rng);
        }
        self.fish_caught += 1;
        self.total_catch_value += fish.value;
        self.update_value_stats(&fish);
        self.inventory.push(fish.clone());
        self.status_message = format!(
            "You caught a {} {} (stored).",
            fish.rarity, fish.species
        );
        self.log(format!(
            "{} | {:.2}kg | {} | value {} (in inventory)",
            fish.species, fish.weight_kg, fish.rarity, fish.value
        ));
        self.ensure_location_access();
        self.pending_fish = None;
    }

    pub fn on_catch_fail(&mut self, rng: &mut impl Rng) {
        self.consume_buff_durations();
        self.casts_since_weather_update += 1;
        if self.casts_since_weather_update >= 4 {
            self.update_world_cycle(rng);
        }
        self.status_message = "The fish escaped during the pull.".to_string();
        self.log("The fish escaped after a bad pull sequence.");
        self.pending_fish = None;
    }

    pub fn rest(&mut self) -> bool {
        if !self.rest_manager.can_rest_now() {
            self.status_message = format!(
                "Rest is on cooldown for {}s.",
                self.rest_manager.seconds_remaining()
            );
            return false;
        }
        self.stamina = (self.stamina + 25).min(MAX_STAMINA);
        let cooldown_ms =
            (38000.0 * self.player_character.cast_speed_multiplier).round() as i64;
        self.rest_manager.trigger_cooldown_ms(cooldown_ms);
        self.status_message = "Rested. Stamina restored. Cooldown started.".to_string();
        true
    }

    pub fn spend_gold(&mut self, base_cost: i32, thing: &str, apply: impl FnOnce(&mut Self)) -> bool {
        let final_cost = self.discounted_cost(base_cost);
        if self.gold < final_cost {
            self.status_message = format!("Not enough gold for {}.", thing);
            return false;
        }
        self.gold -= final_cost;
        apply(self);
        self.status_message = format!("Bought {} for {} gold.", thing, final_cost);
        self.log(format!("Shop: bought {} ({})", thing, final_cost));
        true
    }

    pub fn buy_rod_upgrade(&mut self) -> bool {
        if self.rod_level >= MAX_ROD_LEVEL {
            self.status_message = "Rod already max level.".to_string();
            return false;
        }
        let cost = 200 + self.rod_level * 70;
        self.spend_gold(cost, "Rod upgrade", |w| w.rod_level += 1)
    }

    pub fn buy_bait_upgrade(&mut self) -> bool {
        if self.bait_level >= 5 {
            self.status_message = "Bait already max level.".to_string();
            return false;
        }
        self.spend_gold(140 + self.bait_level * 45, "Bait upgrade", |w| w.bait_level += 1)
    }

    pub fn buy_luck_buff(&mut self) -> bool {
        self.spend_gold(90, "Luck buff", |w| w.luck_buff_casts += 5)
    }

    pub fn buy_value_buff(&mut self) -> bool {
        self.spend_gold(90, "Value buff", |w| w.value_buff_casts += 5)
    }

    pub fn buy_stamina_potion(&mut self) -> bool {
        if self.stamina >= MAX_STAMINA {
            self.status_message = "Stamina already full.".to_string();
            return false;
        }
        self.spend_gold(40, "Stamina potion", |w| {
            w.stamina = (w.stamina + 30).min(MAX_STAMINA)
        })
    }

    pub fn buy_special_rod_mod(&mut self) -> bool {
        if self.special_rod_power >= 4 {
            self.status_message = "Special rod mod already maxed.".to_string();
            return false;
        }
        if self.relation_tinker_kai < 14 {
            self.status_message = "Need stronger relationship with Tinker Kai.".to_string();
            return false;
        }
        let cost = 430 + self.special_rod_power * 140;
        self.spend_gold(cost, "special rod mod", |w| w.special_rod_power += 1)
    }

    pub fn sell_inventory(&mut self) -> bool {
        if self.inventory.is_empty() {
            self.status_message = "Inventory is empty.".to_string();
            return false;
        }
        let sum: i32 = self.inventory.iter().map(|f| f.value).sum();
        self.inventory.clear();
        self.gold += sum;
        self.lifetime_gold_earned += sum;
        self.status_message = format!("Sold inventory in shop for {} gold.", sum);
        self.log(format!("Sold all fish in shop: +{} gold", sum));
        true
    }

    pub fn generate_trade_offer(&mut self, trader: &str, rng: &mut impl Rng) -> bool {
        if self.inventory.is_empty() {
            self.status_message = "Catch fish first before making a trade offer.".to_string();
            return false;
        }
        let min_rarity_options = ["Common", "Uncommon", "Rare"];
        let mut min_rarity = min_rarity_options[rng.random_range(0..min_rarity_options.len())];
        let relation_bonus = (self.relationship(trader) as f64 * 0.01).min(0.30);
        let mut multiplier = 1.35 + rng.random::<f64>() * 0.40 + relation_bonus;
        if trader == "Broker Finn" {
            multiplier += 0.08;
        }
        if trader == "Marina" && min_rarity == "Rare" {
            min_rarity = if rng.random_bool(0.5) {
                "Uncommon"
            } else {
                "Common"
            };
        }
        let expires_at_ms = now_ms() + 45_000;
        let npc = TraderNpc::from_display_name(trader);
        self.trade_offer = Some(TradeOffer {
            trader_name: npc.display_name.to_string(),
            min_rarity: min_rarity.to_string(),
            multiplier,
            expires_at_ms,
        });
        self.status_message = "Trade offer created.".to_string();
        self.log(format!(
            "New trade offer by {}: {}+ fish at x{:.2} for 45s",
            trader, min_rarity, multiplier
        ));
        true
    }

    pub fn try_trade_offer(&mut self) -> bool {
        let offer = match &self.trade_offer {
            Some(o) if o.is_active() => o.clone(),
            _ => {
                self.status_message = "No active trade offer.".to_string();
                self.trade_offer = None;
                return false;
            }
        };

        let min_rank = rarity_rank(&offer.min_rarity);
        let mut sold = 0;
        let mut raw_value = 0;
        self.inventory.retain(|fish| {
            if rarity_rank(&fish.rarity) >= min_rank {
                sold += 1;
                raw_value += fish.value;
                false
            } else {
                true
            }
        });

        if sold == 0 {
            self.status_message = format!(
                "No fish matched trade rarity {}+.",
                offer.min_rarity
            );
            return false;
        }

        let payout = (raw_value as f64 * offer.multiplier).round() as i32;
        self.gold += payout;
        self.lifetime_gold_earned += payout;
        self.add_relationship(&offer.trader_name, 1 + sold / 3);
        self.log(format!(
            "Trade sold {} fish at x{:.2} for +{} gold (Rel {}: {})",
            sold,
            offer.multiplier,
            payout,
            offer.trader_name,
            self.relationship(&offer.trader_name)
        ));
        self.status_message = format!("Trade successful: +{} gold.", payout);
        self.trade_offer = None;
        true
    }

    pub fn trade_text(&self) -> String {
        match &self.trade_offer {
            Some(o) if o.is_active() => format!(
                "Trade x{:.2} for {}+ by {} ({}s left)",
                o.multiplier,
                o.min_rarity,
                o.trader_name,
                o.seconds_remaining()
            ),
            _ => "No trade offer".to_string(),
        }
    }
}
