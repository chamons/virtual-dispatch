use serde::{Deserialize, Serialize};

use crate::{
    mission::{Ice, IceId, Player},
    screen::Screen,
    util::Point,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub name: String,
    #[serde(default)]
    pub ice: Vec<Ice>,
}

impl System {
    pub fn render(&self, screen: &mut Screen, player: &Player) {
        for ice in &self.ice {
            screen.draw_sprite(&ice.sprite, ice.position);
            for output_id in &ice.outputs {
                if let Some(target_node) = self.find_ice(*output_id) {
                    let source_point = ice.position + Point::new(48, 24);
                    let dest_point = target_node.position + Point::new(0, 24);
                    screen.draw_trace(source_point, dest_point);
                }
            }
        }

        screen.render_ice_popup(self.find_player_ice(player), false);
    }

    pub fn find_ice(&self, id: IceId) -> Option<&Ice> {
        self.ice.iter().find(|i| i.id == id)
    }

    pub fn find_player_ice(&self, player: &Player) -> &Ice {
        self.find_ice(player.position)
            .expect("Player must always be at a valid location")
    }
}
