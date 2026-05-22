use crate::{mission::MissionState, prelude::*};

mod mission_ready;
pub use mission_ready::CampaignState;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum CampaignStep {
    MissionReady(CampaignState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignScreenState {
    step: CampaignStep,
    frame: usize,
}

impl CampaignScreenState {
    pub fn new() -> Self {
        Self {
            step: CampaignStep::MissionReady(CampaignState::new()),
            frame: 0,
        }
    }

    pub fn load_save() -> GameFlow {
        match MissionState::load_from_disk() {
            Some(state) => GameFlow::Gameplay(state),
            None => GameFlow::Campaign(CampaignScreenState::new()),
        }
    }

    pub fn process_frame(&mut self, screen: &mut Screen) -> Option<GameFlow> {
        self.frame += 1;

        match &mut self.step {
            CampaignStep::MissionReady(state) => state.process_ready_for_mission(screen),
        }
    }
}
