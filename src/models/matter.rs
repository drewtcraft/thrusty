use super::point::Point;
use super::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Matter {
    pub point: Point,
    pub color: Color,
}

impl Matter {
    pub fn new(point: Point, color: Color) -> Matter {
        Matter {point, color}
    }
}

use std::fmt;
impl fmt::Display for Matter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matter({}, {})", self.point, self.color)
    }
}
