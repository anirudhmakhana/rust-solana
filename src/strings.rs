fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiou".contains(c.to_ascii_lowercase())).count()
}

fn main() {
    println!("{}", count_vowels("Hello, World!"));
    println!("{}", count_vowels("RUST"));
    println!("{}", count_vowels("rhythm"));
}
