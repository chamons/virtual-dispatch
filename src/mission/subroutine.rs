use std::fmt::{Display, Pointer};

use serde::{Deserialize, Serialize};

use crate::mission::Ticks;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Active {
        effect: ActiveEffect,
        #[serde(default)]
        ticks: Ticks,
    },
    Passive(PassiveEffect),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassiveEffect {
    Halt,
}

impl Display for PassiveEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActiveEffect {
    Print,
}

impl Display for ActiveEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subroutine {
    pub name: String,
    pub effect: Effect,
}

impl Display for Subroutine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.effect {
            Effect::Active { effect, ticks } => write!(f, "{effect}: {ticks}"),
            Effect::Passive(passive_effect) => write!(f, "{passive_effect}"),
        }
    }
}
