use std::ops::Index;

use macroquad::input::{KeyCode, is_key_pressed};
use serde::{Deserialize, Serialize};

use crate::{
    mission::{IceId, System},
    screen::Screen,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    frame: usize,
    cursor: IceId,
    target: Option<IceId>,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            frame: 0,
            cursor: IceId(0),
            target: None,
        }
    }

    pub fn render(&mut self, screen: &mut Screen, system: &System) {
        self.frame += 1;

        let cursor_frame = self.frame % 150;
        if cursor_frame < 110 {
            if let Some(ice) = system.find_ice(self.cursor) {
                screen.draw_cursor(ice.position);
            }
        }
    }

    pub fn handle_input(&mut self, system: &System) {
        if is_key_pressed(KeyCode::Left) | is_key_pressed(KeyCode::Kp4) | is_key_pressed(KeyCode::H)
        {
            self.target = self.find_upstream_node(system);
        } else if is_key_pressed(KeyCode::Right)
            | is_key_pressed(KeyCode::Kp6)
            | is_key_pressed(KeyCode::L)
        {
            self.target = self.find_upstream_node(system);
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            self.target = None;
        }
    }

    fn find_upstream_node(&self, system: &System) -> Option<IceId> {
        if let Some(current_ice) = system.find_ice(self.cursor) {
            if let Some(current_target) = self.target {
                if let Some(current_index) = current_ice
                    .inputs
                    .iter()
                    .position(|id| *id == current_target)
                {
                    return current_ice.inputs.get(current_index + 1).cloned();
                }
            } else {
                return current_ice.inputs.first().cloned();
            }
        }
        None
    }
    fn find_downstream_node(&self, system: &System) -> Option<IceId> {
        if let Some(ice) = system.find_ice(self.cursor) {}
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::mission::{
        test_utils::{simple_data, simple_test_system},
        *,
    };

    #[test]
    fn upstream_nodes() {
        let data = simple_data();
        let system = simple_test_system().instance(&data);
        let cursor = Cursor::new();
        for _ in 0..5 {
            println!("{:?}", cursor.find_upstream_node(&system));
        }
    }
}
