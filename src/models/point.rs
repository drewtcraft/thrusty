#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl Point {
    pub fn new(x: u64, y: u64) -> Point {Point {x, y}}
}

use std::fmt;
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}