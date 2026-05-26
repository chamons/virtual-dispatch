use std::collections::HashMap;

use macroquad::{
    audio::{PlaySoundParams, Sound, load_sound, play_sound, set_sound_volume, stop_sound},
    rand::gen_range,
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    text::{draw_text, measure_text},
    texture::{DrawTextureParams, Texture2D, build_textures_atlas, draw_texture_ex},
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
    pub firewall: Texture2D,

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

        let firewall = macroquad::texture::load_texture("resources/art/firewall.png")
            .await
            .expect("Unable to load art");

        build_textures_atlas();

        let camera = Camera::new();
        let options = Options::load();
        Self {
            music,
            camera,
            text,
            firewall,
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

    pub fn draw_trace(&self, start_position: Point, end_position: Point) {
        // Offset the start and end position by the camera
        let start_position = Point::new(
            start_position.x - self.camera.left_x,
            start_position.y - self.camera.top_y,
        );
        let end_position = Point::new(
            end_position.x - self.camera.left_x,
            end_position.y - self.camera.top_y,
        );

        let mid_x = start_position.x as f32 + (end_position.x - start_position.x) as f32 / 2.0;
        // Draw one line half the distance first horizontal
        draw_line(
            start_position.x as f32,
            start_position.y as f32,
            mid_x,
            start_position.y as f32,
            2.0,
            WHITE,
        );
        // Then down
        draw_line(
            mid_x,
            start_position.y as f32,
            mid_x,
            end_position.y as f32,
            2.0,
            WHITE,
        );
        // And now the rest of the way across
        draw_line(
            mid_x,
            end_position.y as f32,
            end_position.x as f32,
            end_position.y as f32,
            2.0,
            WHITE,
        );
    }

    pub fn render_ice_popup(&mut self, ice: &Ice, is_popup: bool) {
        let ice_popup_width = self.ice_popup_width(ice);
        let ice_popup_height = self.ice_popup_height(ice);
        let popup_x = ice.position.x - self.camera.left_x + 40;
        let popup_y = ice.position.y - self.camera.top_y - 10 - ice_popup_height;
        let y_offset = if is_popup { -15 } else { 0 };
        let popup_alpha = if is_popup { 0.4 } else { 0.7 };
        let popup_border = if is_popup { GRAY } else { LIGHTGRAY };

        self.draw_filled_rectangle_with_border(
            Rect::with_size(
                popup_x,
                popup_y + y_offset,
                ice_popup_width,
                ice_popup_height,
            ),
            popup_border,
            Color::new(0.00, 0.47, 0.95, popup_alpha),
        );

        let text_color = if is_popup { LIGHTGRAY } else { WHITE };
        Screen::draw_text_with_color(
            &ice.name,
            19,
            popup_x as f32 + 8.,
            (popup_y + 20 + y_offset) as f32,
            text_color,
        );
    }

    fn ice_popup_width(&self, ice: &Ice) -> i32 {
        let title_width = measure_text(&ice.name, None, 19, 1.0).width;
        title_width as i32 + 16
    }

    fn ice_popup_height(&self, _ice: &Ice) -> i32 {
        40
    }

    pub fn draw_ice_rectangle(&self, position: Point, color: Color) {
        draw_rectangle_lines(
            (position.x - self.camera.left_x) as f32,
            (position.y - self.camera.top_y) as f32,
            48.,
            48.,
            2.,
            color,
        );
    }

    pub fn draw_filled_rectangle_with_border(
        &self,
        rect: Rect,
        outer_color: Color,
        inner_color: Color,
    ) {
        draw_rectangle(
            rect.x1 as f32,
            rect.y1 as f32,
            rect.width() as f32,
            rect.height() as f32,
            inner_color,
        );
        draw_rectangle_lines(
            rect.x1 as f32,
            rect.y1 as f32,
            rect.width() as f32,
            rect.height() as f32,
            4.,
            outer_color,
        );
    }

    pub fn draw_sprite(&self, _sprite: &str, position: Point) {
        let texture = &self.firewall;
        let screen_x: f32 = (position.x - self.camera.left_x) as f32;
        let screen_y: f32 = (position.y - self.camera.top_y) as f32;
        draw_texture_ex(
            texture,
            screen_x,
            screen_y,
            WHITE,
            DrawTextureParams::default(),
        );
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

    pub fn draw_text_with_color(text: &str, size: u16, x: f32, y: f32, text_color: Color) {
        draw_text(text, x, y, size as f32, text_color);
    }

    pub fn draw_line(x: f32, y: f32, width: u32, height: u32, color: Color) {
        macroquad::shapes::draw_line(x, y, x + width as f32, y + height as f32, 1., color);
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
