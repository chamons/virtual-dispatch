use serde::{Deserialize, Serialize};

use crate::{mission::Ice, screen::Screen};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub name: String,
    pub ice: Vec<Ice>,
}

impl System {
    pub fn render(&self, screen: &mut Screen) {
        for ice in &self.ice {
            screen.draw_sprite(&ice.sprite, ice.position);
        }
    }
}
