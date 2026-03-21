fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn main() {
    let (x, y) = swap(1, 2);
    println!("({}, {})", x, y);
}
