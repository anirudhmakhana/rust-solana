// Parsing with slice patterns
// Run: cargo run --bin slice_patterns

fn parse_command(input: &str) -> String {
    // NOTE: split_whitespace() handles multiple spaces and leading/trailing whitespace.
    // collect() into Vec<&str> so we can call .as_slice() for pattern matching.
    let tokens: Vec<&str> = input.split_whitespace().collect();

    match tokens.as_slice() {
        // NOTE: Single-element slice pattern — matches exactly ["quit"].
        // The compiler knows this only fires when there is exactly one token equal to "quit".
        ["quit"] => "Goodbye".to_string(),

        // NOTE: `["echo", rest @ ..]` — fixed head + rest capture.
        // `rest` is bound as a &[&str] of all tokens after "echo" (may be empty).
        ["echo", rest @ ..] => rest.join(" "),

        // NOTE: Exact three-element destructure ["add", x, y].
        // x and y are &str here; we parse them inside a nested match.
        ["add", x, y] => match (x.parse::<i64>(), y.parse::<i64>()) {
            (Ok(a), Ok(b)) => (a + b).to_string(),
            _ => "Unknown".to_string(),
        },

        // NOTE: ["repeat", n, msg @ ..] + match guard.
        // The guard `if !msg.is_empty()` is evaluated AFTER the pattern binds —
        // it ensures "repeat 3" alone (no message) falls through to the wildcard.
        // `if let Ok(count)` inside the arm body handles a non-numeric n gracefully.
        ["repeat", n, msg @ ..] if !msg.is_empty() => {
            if let Ok(count) = n.parse::<usize>() {
                let phrase = msg.join(" ");
                // NOTE: std::iter::repeat yields infinite copies; .take(count) limits it.
                std::iter::repeat(phrase).take(count).collect::<Vec<_>>().join(" ")
            } else {
                "Unknown".to_string()
            }
        }

        // NOTE: Wildcard arm — catches everything not matched above.
        _ => "Unknown".to_string(),
    }
}

fn main() {
    println!("{}", parse_command("quit"));              // Goodbye
    println!("{}", parse_command("echo hello world"));  // hello world
    println!("{}", parse_command("echo"));              // (empty string)
    println!("{}", parse_command("add 3 4"));           // 7
    println!("{}", parse_command("add 10 -2"));         // 8
    println!("{}", parse_command("repeat 3 ha"));       // ha ha ha
    println!("{}", parse_command("repeat 2 hello world")); // hello world hello world
    println!("{}", parse_command("foo bar"));           // Unknown
    println!("{}", parse_command("repeat 3"));          // Unknown
}
