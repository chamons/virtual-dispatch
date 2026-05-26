use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};

// Point and Rect in macroquad use f32
// but that doesn't make any sense for roguelike grids

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn adjacent(&self) -> Vec<Point> {
        [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
            Point::new(-1, 1),
            Point::new(-1, -1),
            Point::new(1, 1),
            Point::new(1, -1),
        ]
        .map(|offset| offset + *self)
        .into_iter()
        .collect()
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self { x1, x2, y1, y2 }
    }

    pub fn with_size(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn width(&self) -> u32 {
        (self.x2 - self.x1) as u32
    }

    pub fn height(&self) -> u32 {
        (self.y2 - self.y1) as u32
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                f(Point::new(x, y));
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timer {
    duration: u32,
    remaining: u32,
    resets: bool,
}

impl Timer {
    pub fn new(duration: u32, resets: bool) -> Self {
        Self {
            duration,
            remaining: duration,
            resets,
        }
    }

    // Returns true when fires
    pub fn tick(&mut self) -> bool {
        if self.remaining == 0 {
            false
        } else {
            self.remaining -= 1;
            let completed = self.remaining == 0;
            if completed && self.resets {
                self.reset();
            }
            completed
        }
    }

    pub fn reset(&mut self) {
        self.remaining = self.duration;
    }
}

// Returns true during the on window and then resets
// during the off window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnOffTimer {
    on: Timer,
    off: Timer,
    is_on: bool,
}

impl OnOffTimer {
    pub fn new(on_duration: u32, off_duration: u32) -> Self {
        OnOffTimer {
            on: Timer::new(on_duration, false),
            off: Timer::new(off_duration, false),
            is_on: true,
        }
    }

    // Returns true when in the on window
    pub fn tick(&mut self) -> bool {
        if self.is_on {
            let on_window_complete = self.on.tick();
            if on_window_complete {
                self.is_on = false;
                self.on.reset();
            }
            true
        } else {
            let off_window_complete = self.off.tick();
            if off_window_complete {
                self.is_on = true;
                self.off.reset();
            }
            false
        }
    }

    pub fn reset(&mut self) {
        self.is_on = true;
        self.on.reset();
        self.off.reset();
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{OnOffTimer, Timer};

    #[test]
    fn timer_fires() {
        let mut timer = Timer::new(2, false);
        assert!(!timer.tick());
        assert!(timer.tick());
        assert!(!timer.tick());
        assert!(!timer.tick());
    }

    #[test]
    fn timer_fires_and_resets() {
        let mut timer = Timer::new(2, true);
        assert!(!timer.tick());
        assert!(timer.tick());
        assert!(!timer.tick());
        assert!(timer.tick());
        assert!(!timer.tick());
        assert!(timer.tick());
    }

    #[test]
    fn timer_reset() {
        let mut timer = Timer::new(2, false);
        assert!(!timer.tick());
        timer.reset();
        assert_eq!(2, timer.remaining);
        assert!(!timer.tick());
        assert!(timer.tick());
    }

    #[test]
    fn on_off_timer_fires() {
        let mut timer = OnOffTimer::new(4, 2);
        assert!(timer.tick());
        assert!(timer.tick());
        assert!(timer.tick());
        assert!(timer.tick());
        assert!(!timer.tick());
        assert!(!timer.tick());
        assert!(timer.tick());
    }

    #[test]
    fn on_off_timer_reset() {
        let mut timer = OnOffTimer::new(3, 1);
        assert!(timer.tick());
        assert!(timer.tick());
        timer.reset();
        assert!(timer.tick());
        assert!(timer.tick());
        assert!(timer.tick());
        assert!(!timer.tick());
        assert!(timer.tick());
    }
}
