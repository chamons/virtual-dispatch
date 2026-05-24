use macroquad::{
    color::{BLUE, Color, GRAY, WHITE},
    input::{KeyCode, is_key_pressed},
};
use serde::{Deserialize, Serialize};

use crate::{
    mission::{IceId, Player, System},
    screen::Screen,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    frame: usize,
    target: Option<IceId>,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            frame: 0,
            target: None,
        }
    }

    pub fn render(&mut self, screen: &mut Screen, system: &System, player: &Player) {
        self.frame += 1;

        let cursor_frame = self.frame % 150;
        let should_draw = cursor_frame < 110;

        if let Some(cursor_target) = self.target {
            if let Some(cursor_ice) = system.find_ice(cursor_target) {
                if should_draw {
                    screen.draw_ice_rectangle(cursor_ice.position, BLUE);
                }
                Screen::draw_text_with_color(
                    "JMP TO?",
                    21,
                    cursor_ice.position.x as f32 - 5.,
                    cursor_ice.position.y as f32 - 5.,
                    BLUE,
                );
            }

            self.draw_cursor(screen, system, player, GRAY);
        } else {
            if should_draw {
                self.draw_cursor(screen, system, player, WHITE);
            }
        }
    }

    fn draw_cursor(&mut self, screen: &mut Screen, system: &System, player: &Player, color: Color) {
        screen.draw_ice_rectangle(system.find_player_ice(player).position, color);
    }

    pub fn handle_input(&mut self, system: &System, player: &mut Player) {
        if is_key_pressed(KeyCode::Left) | is_key_pressed(KeyCode::Kp4) | is_key_pressed(KeyCode::H)
        {
            self.target = self.find_upstream_node(system, player);
        } else if is_key_pressed(KeyCode::Right)
            | is_key_pressed(KeyCode::Kp6)
            | is_key_pressed(KeyCode::L)
        {
            self.target = self.find_downstream_node(system, player);
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            if let Some(target) = self.target {
                player.position = target;
            }
            self.target = None;
        }
    }

    fn find_upstream_node(&self, system: &System, player: &Player) -> Option<IceId> {
        let current_ice = system.find_player_ice(player);

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
        None
    }

    fn find_downstream_node(&self, system: &System, player: &Player) -> Option<IceId> {
        let current_ice = system.find_player_ice(player);
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
        let mut player = Player::new();
        let system = simple_test_system().instance(&data);
        let mut cursor = Cursor::new();

        // First node has only one upstream
        player.position = IceId(0);
        assert_eq!(cursor.find_upstream_node(&system, &player), Some(IceId(2)));
        cursor.target = Some(IceId(2));
        assert_eq!(cursor.find_upstream_node(&system, &player), Some(IceId(2)));

        // Node 2 has two upstream
        player.position = IceId(2);
        cursor.target = None;
        assert_eq!(cursor.find_upstream_node(&system, &player), Some(IceId(0)));
        cursor.target = Some(IceId(0));
        assert_eq!(cursor.find_upstream_node(&system, &player), Some(IceId(3)));
        cursor.target = Some(IceId(3));
        assert_eq!(cursor.find_upstream_node(&system, &player), Some(IceId(0)));

        // Node 4 has no upstream
        player.position = IceId(4);
        cursor.target = None;
        assert_eq!(cursor.find_upstream_node(&system, &player), None);
    }

    #[test]
    fn downstream_nodes() {
        let data = simple_data();
        let system = simple_test_system().instance(&data);
        let mut player = Player::new();
        let mut cursor = Cursor::new();

        // First node has two downstream
        player.position = IceId(0);
        assert_eq!(
            cursor.find_downstream_node(&system, &player),
            Some(IceId(1))
        );
        cursor.target = Some(IceId(1));
        assert_eq!(
            cursor.find_downstream_node(&system, &player),
            Some(IceId(2))
        );
        cursor.target = Some(IceId(2));
        assert_eq!(
            cursor.find_downstream_node(&system, &player),
            Some(IceId(1))
        );

        player.position = IceId(2);
        cursor.target = None;
        assert_eq!(
            cursor.find_downstream_node(&system, &player),
            Some(IceId(0))
        );
        cursor.target = Some(IceId(0));
        assert_eq!(
            cursor.find_downstream_node(&system, &player),
            Some(IceId(0))
        );

        // Node 4 has no downstream
        player.position = IceId(4);
        cursor.target = None;
        assert_eq!(cursor.find_downstream_node(&system, &player), None);
    }
}
