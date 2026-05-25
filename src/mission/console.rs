use macroquad::color::{BLUE, Color, DARKBLUE, DARKGRAY, WHITE};
use serde::{Deserialize, Serialize};

use crate::{
    prelude::{SCREEN_HEIGHT, SCREEN_WIDTH},
    screen::Screen,
    util::{Point, Rect},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Console {
    logs: Vec<String>,
    current_command: String,
}

impl Console {
    pub fn new() -> Self {
        Self {
            logs: (0..20).map(|i| i.to_string()).collect(),
            // logs: vec![],
            current_command: String::new(),
        }
    }

    pub fn render(&self, screen: &mut Screen) {
        const CONSOLE_TOP: i32 = 600;

        screen.draw_filled_rectangle_with_border(
            Rect::new(0, SCREEN_WIDTH, CONSOLE_TOP, SCREEN_HEIGHT),
            Color::new(0.51, 0.51, 0.51, 0.7),
            Color::new(0.00, 0.32, 0.67, 0.99),
        );

        for (i, line) in self.logs.iter().rev().take(8).rev().enumerate() {
            Screen::draw_text_with_color(
                &format!("> {line}"),
                21,
                8.,
                CONSOLE_TOP as f32 + 21. + i as f32 * 20.,
                WHITE,
            );
        }

        Screen::draw_line(
            5.,
            CONSOLE_TOP as f32 + 171.,
            SCREEN_WIDTH as u32 - 10,
            1,
            WHITE,
        );

        Screen::draw_text_with_color(
            &format!("$ {}", self.current_command),
            21,
            8.,
            CONSOLE_TOP as f32 + 190.,
            WHITE,
        );
    }

    pub fn push_command_to_log(&mut self) {
        self.logs.push(self.current_command.clone());
        self.current_command = String::new();
    }

    pub fn update_current_command(&mut self, command: String) {
        self.current_command = command;
    }

    pub fn clear_current_command(&mut self) {
        self.current_command = String::new();
    }
}
