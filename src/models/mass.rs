use super::matter::Matter;
use super::point::Point;

pub struct Mass {
    pub top_left: Point,
    pub matter: Vec<Matter>,
}

impl Mass {
    pub fn new(top_left: Point, matter: Vec<Matter>) -> Mass {
        Mass {top_left, matter}
    }
}
