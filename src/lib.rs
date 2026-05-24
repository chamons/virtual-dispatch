use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod campaign;
mod flow;
pub mod mission;
mod screen;
pub mod screens;
mod util;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Normal,
    Easy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub music: f32,
    pub sound: f32,
    pub difficulty: Difficulty,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            music: 0.35,
            sound: 0.3,
            difficulty: Difficulty::Normal,
        }
    }
}

impl Options {
    pub fn load() -> Options {
        if let Ok(text) = std::fs::read(&Self::options_path()) {
            serde_json::from_slice(&text).unwrap_or_default()
        } else {
            Options::default()
        }
    }

    #[cfg(feature = "desktop")]
    pub fn save(&self) {
        let filename = Self::options_path();

        match std::fs::create_dir_all(filename.parent().expect("Project dir should be longer")) {
            Ok(()) => {
                if let Err(e) = std::fs::write(
                    filename,
                    serde_json::to_string(self).expect("Unable to save options"),
                ) {
                    eprintln!("Unable to save options: {e:?}");
                }
            }
            Err(e) => {
                eprintln!("Unable to create options location: {e:?}");
            }
        }
    }

    #[cfg(not(feature = "desktop"))]
    pub fn save(&self) {}

    #[cfg(feature = "desktop")]
    pub fn options_path() -> PathBuf {
        let dirs = directories::ProjectDirs::from("com", "", "VirtualDispatch")
            .expect("Unable to find project directory?");
        let mut path = dirs.data_dir().to_path_buf();
        path.push("options.json");
        path
    }

    #[cfg(not(feature = "desktop"))]
    pub fn options_path() -> PathBuf {
        PathBuf::new()
    }
}

pub mod prelude {
    pub use crate::flow::*;
    pub use crate::screen::*;
    pub use crate::util::*;

    pub use macroquad::color::*;
    pub use macroquad::input::{KeyCode, is_key_pressed};
    pub use macroquad::math::Rect as MRect;
    pub use macroquad::rand;
    pub use macroquad::rand::RandGenerator;

    pub use serde::{Deserialize, Serialize};

    pub const SCREEN_WIDTH: i32 = 1024;
    pub const SCREEN_HEIGHT: i32 = 800;

    pub const CAMERA_VIEWPORT_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const CAMERA_VIEWPORT_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub const TICKS_FLOATING_TEXT: u32 = 120;

    pub const VERSION: &str = "0.01";
}
