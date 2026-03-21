fn join_strings(a: String, b: String) -> String {
    a + " " + &b
}

fn main() {
    let a = String::from("Hello");
    let b = String::from("Rust");
    println!("{}", join_strings(a, b));
}
