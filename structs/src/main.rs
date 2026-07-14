#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn sum(&self, other: &Point) -> Point {
        dbg!(self, other);
        Point {
            x: dbg!(self.x + other.x),
            y: dbg!(self.y + other.y),
        }
    }
}

fn main() {

    let p1 = Point::new(10, 20);
    let p2 = Point::new(1, 2);
    let p3 = p1.sum(&p2);

    println!("{p3:#?}");
}
