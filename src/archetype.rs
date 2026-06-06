#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerArchetype {
    pub display_name: &'static str,
    pub starting_gold: i32,
    pub starting_rod_level: i32,
    pub starting_bait_level: i32,
    pub starting_stamina: i32,
    pub cast_speed_multiplier: f64,
    pub relation_gain_multiplier: f64,
    pub shop_cost_multiplier: f64,
}

impl PlayerArchetype {
    pub const ALEX: Self = Self {
        display_name: "Alex",
        starting_gold: 90,
        starting_rod_level: 0,
        starting_bait_level: 1,
        starting_stamina: 100,
        cast_speed_multiplier: 1.00,
        relation_gain_multiplier: 1.00,
        shop_cost_multiplier: 1.00,
    };

    pub const TOMMY: Self = Self {
        display_name: "Tommy",
        starting_gold: 70,
        starting_rod_level: 0,
        starting_bait_level: 1,
        starting_stamina: 100,
        cast_speed_multiplier: 0.96,
        relation_gain_multiplier: 1.12,
        shop_cost_multiplier: 0.98,
    };

    pub const RYAN: Self = Self {
        display_name: "Ryan",
        starting_gold: 120,
        starting_rod_level: 0,
        starting_bait_level: 1,
        starting_stamina: 95,
        cast_speed_multiplier: 1.05,
        relation_gain_multiplier: 1.00,
        shop_cost_multiplier: 0.92,
    };

    pub const ALL: [Self; 3] = [Self::ALEX, Self::TOMMY, Self::RYAN];

    pub fn from_name(name: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|a| a.display_name.eq_ignore_ascii_case(name))
            .unwrap_or(Self::ALEX)
    }
}
