use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let a = Point { x: 1, y: 0 };
    let b = Point { x: 2, y: 3 };
    let c = Point { x: 3, y: 3 };
    assert_eq!(a + b, c);
}
