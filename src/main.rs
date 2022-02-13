mod models;
mod types;

use models::{Point, Color, Matter};
use types::Direction;


fn main() {
    let p = Point::new(10, 20);
    let d = Direction::TopLeft;
    let c = Color::new(100, 120, 130);
    let m = Matter::new(p, c);
    println!("hellp matter {}!", m);
}
