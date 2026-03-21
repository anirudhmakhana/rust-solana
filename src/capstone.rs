use std::collections::HashMap;

fn most_frequent(s: &str) -> String {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in s.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    let max_count = *counts.values().max().unwrap();

    s.split_whitespace()
        .map(|w| w.to_lowercase())
        .find(|w| counts[w.as_str()] == max_count)
        .unwrap()
}

fn main() {
    println!("{}", most_frequent("the cat sat on the mat the cat"));
    println!("{}", most_frequent("one two two three three three"));
    println!("{}", most_frequent("tie tie other other"));
}
