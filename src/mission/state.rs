use std::path::PathBuf;

use macroquad::input::{is_key_down, is_quit_requested};

use crate::campaign::CampaignState;
use crate::mission::{Cursor, Data, Player, System};
use crate::prelude::*;
use crate::screens::help::HelpState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionState {
    pub frame: usize,
    pub mission_complete: bool,
    pub campaign: CampaignState,
    pub system: System,
    pub player: Player,
    pub cursor: Cursor,
}

impl MissionState {
    pub fn new(campaign: CampaignState, name: &str) -> MissionState {
        let data = Data::load().expect("Unable to load data");
        let system = data.get_system_info(name).instance(&data);

        Self {
            frame: 0,
            mission_complete: false,
            campaign,
            system,
            cursor: Cursor::new(),
            player: Player::new(),
        }
    }

    pub fn process_frame(&mut self, screen: &mut Screen) -> Option<GameFlow> {
        self.frame += 1;

        loop {
            if cfg!(feature = "desktop") {
                if is_quit_requested()
                    || (is_key_pressed(KeyCode::Q)
                        && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)))
                {
                    self.save_to_disk();
                    return Some(GameFlow::Quitting);
                }
            }
            if is_key_pressed(KeyCode::H) {
                return Some(GameFlow::Help(HelpState::new(GameFlow::Gameplay(
                    self.clone(),
                ))));
            }

            #[cfg(debug_assertions)]
            if is_key_pressed(KeyCode::F1) {
                self.process_debug_request(DebugRequest::Save, screen);
            }
            #[cfg(debug_assertions)]
            if is_key_pressed(KeyCode::F2) {
                self.process_debug_request(DebugRequest::Load, screen);
            }

            self.cursor.handle_input(&self.system, &mut self.player);

            self.system.render(screen);
            self.cursor.render(screen, &self.system, &self.player);

            screen.render_floating_text();

            break;
        }

        None
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebugRequest {
    Save,
    Load,
}

impl MissionState {
    fn process_debug_request(&mut self, action: DebugRequest, screen: &mut Screen) {
        screen.push_floating_text(&format!("Running debug command: {action:?}"));
        match action {
            DebugRequest::Save => {
                std::fs::write("dev.save", self.save_to_string()).expect("Unable to save");
            }
            DebugRequest::Load => {
                if let Ok(text) = std::fs::read("dev.save") {
                    *self = serde_json::from_slice(&text).expect("Unable to load dev save");
                }
            }
        }
    }

    pub fn save_to_string(&self) -> String {
        serde_json::to_string(self).expect("Unable to save game")
    }

    #[cfg(feature = "desktop")]
    pub fn savefile_name() -> PathBuf {
        let dirs = directories::ProjectDirs::from("com", "", "VirtualDispatch")
            .expect("Unable to find project directory?");
        let mut path = dirs.data_dir().to_path_buf();
        path.push("game.sav");
        path
    }

    #[cfg(not(feature = "desktop"))]
    pub fn savefile_name() -> PathBuf {
        PathBuf::new()
    }

    pub fn savefile_exists() -> bool {
        let filename = Self::savefile_name();

        match std::fs::exists(&filename) {
            Ok(exists) => exists,
            Err(_) => false,
        }
    }

    pub fn save_to_disk(&self) {
        let filename = Self::savefile_name();

        match std::fs::create_dir_all(filename.parent().expect("Project dir should be longer")) {
            Ok(()) => {
                if let Err(e) = std::fs::write(filename, self.save_to_string()) {
                    eprintln!("Unable to save game: {e:?}");
                }
            }
            Err(e) => {
                eprintln!("Unable to create game location: {e:?}");
            }
        }
    }

    pub fn delete_any_save() {
        let filename = Self::savefile_name();
        if std::fs::remove_file(filename).is_err() {
            eprintln!("Unable to delete game after load.");
        }
    }

    pub fn load_from_disk() -> Option<Self> {
        let filename = Self::savefile_name();
        if let Ok(text) = std::fs::read(&filename) {
            match serde_json::from_slice(&text) {
                Ok(state) => {
                    Self::delete_any_save();
                    return Some(state);
                }
                Err(e) => {
                    eprintln!("Unable to load game: {e:?}");
                }
            }
        }

        None
    }
}
