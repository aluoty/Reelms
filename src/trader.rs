use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraderNpc {
    pub display_name: &'static str,
    dialogue_pool: &'static [&'static str],
}

impl TraderNpc {
    pub const MARINA: Self = Self {
        display_name: "Marina",
        dialogue_pool: &[
            "Fresh fish, fair tides. Bring me quality and I pay back.",
            "Storm is coming. Trade now while prices are hot.",
            "You reel clean. I like reliable anglers.",
        ],
    };

    pub const BROKER_FINN: Self = Self {
        display_name: "Broker Finn",
        dialogue_pool: &[
            "Margins matter. Bring me rarity and we both profit.",
            "Quick hands, quick deal. Show me your best lot.",
            "You keep the supply flowing, I keep the rates flowing.",
        ],
    };

    pub const TINKER_KAI: Self = Self {
        display_name: "Tinker Kai",
        dialogue_pool: &[
            "I tune rods that sing through the line.",
            "Bring materials and trust; I can upgrade miracles.",
            "You fish hard. I can make your rod hit harder.",
        ],
    };

    pub const ALL: [Self; 3] = [Self::MARINA, Self::BROKER_FINN, Self::TINKER_KAI];

    pub fn random_dialogue(&self, rng: &mut impl Rng) -> &'static str {
        let idx = rng.random_range(0..self.dialogue_pool.len());
        self.dialogue_pool[idx]
    }

    pub fn from_display_name(name: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|t| t.display_name.eq_ignore_ascii_case(name))
            .unwrap_or(Self::MARINA)
    }
}
