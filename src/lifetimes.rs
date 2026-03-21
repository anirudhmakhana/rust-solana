fn classify(n: i32) -> &'static str {
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    }
}

fn main() {
    for n in [5, -3, 0] {
        println!("{}: {}", n, classify(n));
    }
}
