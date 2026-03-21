fn first_word(s: &str) -> String {
    s.split_whitespace()
        .next()
        .unwrap_or(s)
        .to_string()
}

fn main() {
    println!("{}", first_word("hello world"));
    println!("{}", first_word("rust"));
    println!("{}", first_word("  leading spaces"));
}
