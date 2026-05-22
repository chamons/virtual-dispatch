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

    pub fn update(&mut self, center: Point) {
        self.left_x = center.x - CAMERA_VIEWPORT_WIDTH / 2;
        self.right_x = center.x + CAMERA_VIEWPORT_WIDTH / 2;
        self.top_y = center.y - CAMERA_VIEWPORT_HEIGHT / 2;
        self.bottom_y = center.y + CAMERA_VIEWPORT_HEIGHT / 2;
    }

    pub fn is_in_view(&self, point: Point) -> bool {
        let viewport = Rect::new(self.left_x, self.right_x, self.top_y, self.bottom_y);
        viewport.contains(point)
    }
}
