use std::collections::HashMap;

use macroquad::{
    audio::{PlaySoundParams, Sound, load_sound, play_sound, set_sound_volume, stop_sound},
    rand::gen_range,
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    texture::{Texture2D, build_textures_atlas},
    window::screen_width,
};

use crate::prelude::*;
use crate::{Options, mission::*};

pub struct FloatingText {
    pub text: String,
    pub timer: u32,
}

pub struct Music {
    tracks: Vec<Sound>,
    sounds: HashMap<String, Sound>,
    current_track: Option<usize>,
    current_sound: Option<String>,
}

impl Music {
    pub fn new() -> Self {
        Self {
            tracks: vec![],
            sounds: HashMap::new(),
            current_track: None,
            current_sound: None,
        }
    }

    pub async fn load(&mut self) {
        self.tracks = vec![];

        self.sounds.insert(
            "drip".to_string(),
            load_sound("resources/sound/drip.ogg")
                .await
                .expect("Unable to load sound"),
        );
    }

    pub fn play_music_track(&mut self, index: usize, options: &Options) {
        self.play(index, options);
    }

    pub fn play_random_music(&mut self, options: &Options) {
        let track = gen_range(1, self.tracks.len());
        self.play(track, options);
    }

    pub fn play_sound(&mut self, name: &str, options: &Options) {
        if let Some(current_sound) = &self.current_sound {
            stop_sound(&self.sounds.get(current_sound).expect("Unable to get sound"));
        }

        let sound = self.sounds.get(name).expect("Unable to get sound");

        play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume: options.sound,
            },
        );
    }

    fn play(&mut self, _index: usize, _options: &Options) {
        if let Some(current_track) = &self.current_track {
            stop_sound(&self.tracks[*current_track]);
        }

        // When we have music

        // let track = &self.tracks[index];

        // play_sound(
        //     &track,
        //     PlaySoundParams {
        //         looped: true,
        //         volume: options.music,
        //     },
        // );
        // self.current_track = Some(index);
    }

    fn set_music_volume(&mut self, volume: f32) {
        if let Some(current_track) = &self.current_track {
            let current_track = &self.tracks[*current_track];

            set_sound_volume(current_track, volume);
        }
    }

    fn current_music_track(&mut self) -> Option<usize> {
        self.current_track
    }
}

// So we can test things with out sounds
pub trait ScreenInterface {
    fn current_music_track(&mut self) -> Option<usize>;

    fn play_music_track(&mut self, index: usize);

    fn play_random_music(&mut self);

    fn set_music_volume(&mut self, volume: f32);

    fn play_sound(&mut self, name: &str);
}

#[cfg(test)]
pub struct EmptyScreen {}

#[cfg(test)]
impl ScreenInterface for EmptyScreen {
    fn play_music_track(&mut self, _index: usize) {}

    fn play_random_music(&mut self) {}

    fn set_music_volume(&mut self, _volume: f32) {}

    fn play_sound(&mut self, _name: &str) {}

    fn current_music_track(&mut self) -> Option<usize> {
        None
    }
}

pub struct Screen {
    pub camera: Camera,
    pub text: Texture2D,

    pub floating_text: Option<FloatingText>,
    music: Music,
    pub options: Options,
}

impl Screen {
    pub async fn new() -> Self {
        let music = Music::new();

        let text = macroquad::texture::load_texture("resources/art/terminal8x8.png")
            .await
            .expect("Unable to load art");

        build_textures_atlas();

        let camera = Camera::new();
        let options = Options::load();
        Self {
            music,
            camera,
            text,
            floating_text: None,
            options,
        }
    }

    pub fn push_floating_text(&mut self, text: &str) {
        self.floating_text = Some(FloatingText {
            text: text.to_string(),
            timer: TICKS_FLOATING_TEXT,
        });
    }

    pub fn push_extended_floating_text(&mut self, text: &str) {
        self.floating_text = Some(FloatingText {
            text: text.to_string(),
            timer: TICKS_FLOATING_TEXT * 3,
        });
    }

    pub fn render_floating_text(&mut self) {
        if let Some(floating_text) = &mut self.floating_text {
            floating_text.timer -= 1;
            if floating_text.timer == 0 {
                self.floating_text = None;
            } else {
                let foreground_fade = (20 + floating_text.timer as i32).min(60) as f32 / 60.0;
                let text_color = Color::new(
                    1.00 * foreground_fade,
                    1.00 * foreground_fade,
                    1.00 * foreground_fade,
                    1.00,
                );

                let background_fade = (floating_text.timer as i32).min(15) as f32 / 15.0;
                let background = Color::new(0.0, 0.0, 0.0, 1.00 * background_fade);
                Self::draw_centered_text_with_color(
                    &floating_text.text,
                    21,
                    53.0,
                    text_color,
                    Some(background),
                );
            }
        }
    }

    pub fn draw_centered_text(text: &str, size: u16, y: f32, background: Option<Color>) {
        Self::draw_centered_text_with_color(text, size, y, WHITE, background);
    }

    pub fn draw_centered_text_with_color(
        text: &str,
        size: u16,
        y: f32,
        text_color: Color,
        background: Option<Color>,
    ) {
        let text_size = measure_text(text, None, size, 1.0);
        let text_x = screen_width() / 2.0 - text_size.width / 2.0;

        if let Some(background) = background {
            const BACKGROUND_PADDING: f32 = 2.0;

            draw_rectangle(
                text_x - BACKGROUND_PADDING,
                y - text_size.offset_y - BACKGROUND_PADDING,
                text_size.width + BACKGROUND_PADDING * 2.0,
                text_size.height + BACKGROUND_PADDING * 2.0,
                background,
            );
        }

        draw_text(text, text_x, y, size as f32, text_color);
    }

    pub async fn load(&mut self) {
        self.music.load().await;
    }
}

impl ScreenInterface for Screen {
    fn current_music_track(&mut self) -> Option<usize> {
        self.music.current_music_track()
    }

    fn play_music_track(&mut self, index: usize) {
        self.music.play_music_track(index, &self.options);
    }

    fn play_random_music(&mut self) {
        self.music.play_random_music(&self.options);
    }

    fn play_sound(&mut self, name: &str) {
        self.music.play_sound(name, &self.options);
    }

    fn set_music_volume(&mut self, volume: f32) {
        self.music.set_music_volume(volume);
    }
}
