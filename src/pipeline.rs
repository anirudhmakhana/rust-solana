// NOTE: A trait with a single method — each implementor is one transformation step.
// This is the "strategy pattern" in Rust: behaviour is swappable at runtime via dyn Formatter.
trait Formatter {
    fn format(&self, input: &str) -> String;
}

// NOTE: Unit structs — no fields, zero bytes of data. They exist only as types.
// We only need them as tags to hang the impl on.
struct Upper;
struct Snake;
struct Trim;

impl Formatter for Upper {
    fn format(&self, input: &str) -> String {
        // NOTE: to_uppercase() returns a new String — it does not mutate the original.
        input.to_uppercase()
    }
}

impl Formatter for Snake {
    fn format(&self, input: &str) -> String {
        // NOTE: replace() scans the string for the first arg and substitutes the second.
        // Returns a new owned String.
        input.replace(' ', "_")
    }
}

impl Formatter for Trim {
    fn format(&self, input: &str) -> String {
        // NOTE: trim() returns a &str (a slice of the original), so we call .to_string()
        // to produce an owned String that matches the return type.
        input.trim().to_string()
    }
}

// NOTE: fold() is the right tool here — it threads a value through a sequence of transformations.
// acc starts as the input string, and each step produces the input for the next step.
// This is a classic pipeline: output of step N becomes input to step N+1.
fn apply_all(input: &str, fmts: &[Box<dyn Formatter>]) -> String {
    fmts.iter().fold(input.to_string(), |acc, f| f.format(&acc))
}

fn main() {
    let fmts: Vec<Box<dyn Formatter>> = vec![
        Box::new(Trim),
        Box::new(Upper),
        Box::new(Snake),
    ];

    // NOTE: Each formatter runs in order — Trim first, then Upper, then Snake.
    // "  hello world  " → "hello world" → "HELLO WORLD" → "HELLO_WORLD"
    println!("{}", apply_all("  hello world  ", &fmts));
}
