fn get_grade(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

fn main() {
    for score in [95, 83, 74, 61, 45] {
        println!("{}: {}", score, get_grade(score));
    }
}
