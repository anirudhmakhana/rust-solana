#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn are_equal(a: &Point, b: &Point) -> bool {
    a == b
}

fn distance_sq(a: &Point, b: &Point) -> i32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 4, y: 6 };

    println!("{}", are_equal(&p1, &p2));
    println!("{}", are_equal(&p1, &p3));
    println!("{}", distance_sq(&p1, &p3));
}
