use macroquad::{
    color::{BLUE, Color, GRAY, WHITE},
    input::{KeyCode, is_key_pressed},
};
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
        let should_draw = cursor_frame < 110;

        if let Some(cursor_target) = self.target {
            if let Some(ice) = system.find_ice(cursor_target) {
                if should_draw {
                    screen.draw_ice_rectangle(ice.position, BLUE);
                }
                Screen::draw_text_with_color(
                    "JMP TO?",
                    21,
                    ice.position.x as f32 - 5.,
                    ice.position.y as f32 - 5.,
                    BLUE,
                );
            }

            self.draw_cursor(screen, system, GRAY);
        } else {
            if should_draw {
                self.draw_cursor(screen, system, WHITE);
            }
        }
    }

    fn draw_cursor(&mut self, screen: &mut Screen, system: &System, color: Color) {
        if let Some(ice) = system.find_ice(self.cursor) {
            screen.draw_ice_rectangle(ice.position, color);
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
            self.target = self.find_downstream_node(system);
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            if let Some(target) = self.target {
                self.cursor = target;
            }
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
                    // Get the next in the list looping around if we hit the end
                    return match current_ice.inputs.get(current_index + 1) {
                        Some(next) => Some(*next),
                        None => current_ice.inputs.first().cloned(),
                    };
                }
            } else {
                return current_ice.inputs.first().cloned();
            }
        }
        None
    }
    fn find_downstream_node(&self, system: &System) -> Option<IceId> {
        if let Some(current_ice) = system.find_ice(self.cursor) {
            if let Some(current_target) = self.target {
                if let Some(current_index) = current_ice
                    .outputs
                    .iter()
                    .position(|id| *id == current_target)
                {
                    // Get the next in the list looping around if we hit the end
                    return match current_ice.outputs.get(current_index + 1) {
                        Some(next) => Some(*next),
                        None => current_ice.outputs.first().cloned(),
                    };
                }
            } else {
                return current_ice.outputs.first().cloned();
            }
        }
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
        let mut cursor = Cursor::new();

        // First node has only one upstream
        cursor.cursor = IceId(0);
        assert_eq!(cursor.find_upstream_node(&system), Some(IceId(2)));
        cursor.target = Some(IceId(2));
        assert_eq!(cursor.find_upstream_node(&system), Some(IceId(2)));

        // Node 2 has two upstream
        cursor.cursor = IceId(2);
        cursor.target = None;
        assert_eq!(cursor.find_upstream_node(&system), Some(IceId(0)));
        cursor.target = Some(IceId(0));
        assert_eq!(cursor.find_upstream_node(&system), Some(IceId(3)));
        cursor.target = Some(IceId(3));
        assert_eq!(cursor.find_upstream_node(&system), Some(IceId(0)));

        // Node 4 has no upstream
        cursor.cursor = IceId(4);
        cursor.target = None;
        assert_eq!(cursor.find_upstream_node(&system), None);
    }

    #[test]
    fn downstream_nodes() {
        let data = simple_data();
        let system = simple_test_system().instance(&data);
        let mut cursor = Cursor::new();

        // First node has two downstream
        cursor.cursor = IceId(0);
        assert_eq!(cursor.find_downstream_node(&system), Some(IceId(1)));
        cursor.target = Some(IceId(1));
        assert_eq!(cursor.find_downstream_node(&system), Some(IceId(2)));
        cursor.target = Some(IceId(2));
        assert_eq!(cursor.find_downstream_node(&system), Some(IceId(1)));

        cursor.cursor = IceId(2);
        cursor.target = None;
        assert_eq!(cursor.find_downstream_node(&system), Some(IceId(0)));
        cursor.target = Some(IceId(0));
        assert_eq!(cursor.find_downstream_node(&system), Some(IceId(0)));

        // Node 4 has no downstream
        cursor.cursor = IceId(4);
        cursor.target = None;
        assert_eq!(cursor.find_downstream_node(&system), None);
    }
}
