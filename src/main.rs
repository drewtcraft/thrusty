mod models;
mod types;

use models::Point;
use types::Direction;

fn main() {
    let p = Point::new(10, 20);
    println!("Hello, point {}", p);

    let d = Direction::TopLeft;
    println!("hello direction {}", d);
}
