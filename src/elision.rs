fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    if s.starts_with(prefix) {
        &s[prefix.len()..]
    } else {
        s
    }
}

fn main() {
    println!("{}", trim_prefix("hello world", "hello "));
    println!("{}", trim_prefix("hello world", "bye"));
}
