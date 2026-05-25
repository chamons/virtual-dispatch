use crate::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            left_x: 0,
            right_x: 0,
            top_y: 0,
            bottom_y: 0,
        }
    }

    pub fn point_centered(&mut self, center: Point) {
        self.left_x = center.x - SCREEN_WIDTH / 2;
        self.right_x = center.x + SCREEN_WIDTH / 2;
        self.top_y = center.y - SCREEN_HEIGHT / 2;
        self.bottom_y = center.y + SCREEN_HEIGHT / 2;
    }

    pub fn scroll(&mut self, delta: Point) {
        self.left_x += delta.x;
        self.right_x += delta.x;
        self.top_y += delta.y;
        self.bottom_y += delta.y;
    }

    pub fn is_in_view(&self, point: Point) -> bool {
        let viewport = Rect::new(self.left_x, self.right_x, self.top_y, self.bottom_y);
        viewport.contains(point)
    }
}
