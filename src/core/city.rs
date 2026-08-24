//! Default City implementation.

use super::point::Point;

/// Default city implementation with x and y coordinates.
#[derive(Clone, Copy, Debug)]
pub struct City {
    pub x: f64,
    pub y: f64,
}

impl Point for City {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}
