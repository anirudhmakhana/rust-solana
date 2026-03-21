fn transform(s: &str) -> i32 {
    let s = s.trim();
    let s: i32 = s.parse().unwrap();
    let s = s * s;
    s
}

fn main() {
    println!("{}", transform("  7  "));
}
