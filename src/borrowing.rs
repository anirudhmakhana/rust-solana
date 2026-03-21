fn count_chars(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let s = String::from("Hello, Rust!");
    println!("{}", count_chars(&s));
    println!("{}", count_chars("café"));
}
