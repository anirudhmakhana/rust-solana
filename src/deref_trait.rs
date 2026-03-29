use std::ops::Deref;

// NOTE: Tuple struct — single field accessed as self.0
// Wrapper<T> is generic over T, so it works with String, i32, Vec, anything.
struct Wrapper<T>(T);

// NOTE: Implementing Deref lets Rust treat &Wrapper<T> as &T automatically.
// `type Target = T` is the associated type — tells Rust what we deref *to*.
// `deref()` returns a reference to the inner value (self.0).
// We never call .deref() manually — Rust calls it behind the scenes.
impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// NOTE: This function expects &str — a plain string slice.
// We will call it with &Wrapper<String>.
// Deref coercion chain: &Wrapper<String> → &String → &str (two hops, automatic).
// Rust applies as many deref steps as needed to satisfy the type.
fn double_len(s: &str) -> usize {
    s.len() * 2
}

fn main() {
    let w = Wrapper(String::from("hello"));

    // NOTE: &w is &Wrapper<String>.
    // Rust coerces: &Wrapper<String> --Deref--> &String --Deref--> &str
    // double_len receives &str without us writing anything explicit.
    println!("{}", double_len(&w)); // 10
}
