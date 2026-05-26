use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ticks {
    ticks: u32,
}

impl Display for Ticks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ticks)
    }
}

impl Ticks {
    pub fn new() -> Self {
        Ticks { ticks: 0 }
    }

    pub fn tick(&mut self, amount: u32) {
        self.ticks += amount;
    }

    pub fn ready(&self, cost: u32) -> bool {
        self.ticks >= cost
    }

    pub fn charge(&mut self, cost: u32) {
        self.ticks -= cost;
    }
}
