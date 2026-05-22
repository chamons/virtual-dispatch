use crate::prelude::*;

pub fn handle_movement_key() -> Option<Point> {
    if is_key_pressed(KeyCode::Left) | is_key_pressed(KeyCode::Kp4) | is_key_pressed(KeyCode::H) {
        Some(Point::new(-1, 0))
    } else if is_key_pressed(KeyCode::Right)
        | is_key_pressed(KeyCode::Kp6)
        | is_key_pressed(KeyCode::L)
    {
        Some(Point::new(1, 0))
    } else if is_key_pressed(KeyCode::Up)
        | is_key_pressed(KeyCode::Kp8)
        | is_key_pressed(KeyCode::K)
    {
        Some(Point::new(0, -1))
    } else if is_key_pressed(KeyCode::Down)
        | is_key_pressed(KeyCode::Kp2)
        | is_key_pressed(KeyCode::J)
    {
        Some(Point::new(0, 1))
    } else if is_key_pressed(KeyCode::Kp1) | is_key_pressed(KeyCode::B) {
        Some(Point::new(-1, 1))
    } else if is_key_pressed(KeyCode::Kp3) | is_key_pressed(KeyCode::N) {
        Some(Point::new(1, 1))
    } else if is_key_pressed(KeyCode::Kp7) | is_key_pressed(KeyCode::Y) {
        Some(Point::new(-1, -1))
    } else if is_key_pressed(KeyCode::Kp9) | is_key_pressed(KeyCode::U) {
        Some(Point::new(1, -1))
    } else {
        None
    }
}
