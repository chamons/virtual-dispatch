use crate::{mission::MissionState, prelude::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {}

impl CampaignState {
    pub fn new() -> CampaignState {
        Self {}
    }

    pub fn process_ready_for_mission(&mut self, screen: &mut Screen) -> Option<GameFlow> {
        screen.play_random_music();

        Some(GameFlow::Gameplay(MissionState::new(self.clone())))
    }

    pub fn game_complete(&self) -> bool {
        false
    }
}
