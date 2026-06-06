use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fish {
    pub species: String,
    pub rarity: String,
    pub weight_kg: f64,
    pub value: i32,
}
