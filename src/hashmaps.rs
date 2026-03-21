use std::collections::HashSet;

fn unique_word_count(s: &str) -> usize {
    s.split_whitespace().collect::<HashSet<_>>().len()
}

fn main() {
    println!("{}", unique_word_count("the cat sat on the mat"));
    println!("{}", unique_word_count("one two three"));
}
