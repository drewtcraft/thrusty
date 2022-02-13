#[derive(Clone, Copy, Debug)]
pub struct Color {
    h: u8,
    s: u8,
    v: u8,
}

impl Color {
    pub fn new(h: u8, s: u8, v: u8) -> Color {Color {h, s, v}}
}

use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.h, self.s, self.v)
    }
}
