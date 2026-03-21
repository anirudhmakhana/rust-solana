fn abs_value(n: i32) -> i32 {
    if n < 0 { -n } else { n }
}

fn main() {
    println!("{}", abs_value(-5));
    println!("{}", abs_value(3));
}
