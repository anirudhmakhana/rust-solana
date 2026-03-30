fn classify(n: i32) -> String {
    match n {
        // NOTE: Exact value match — no binding needed, just the literal 0.
        0 => "zero".to_string(),

        // NOTE: `n @ 1..=10` does two things at once:
        //   1. Tests whether n falls in the range 1..=10
        //   2. Binds the matched value to `n` so we can use it in the arm body
        // Without @, a range pattern would match but you'd lose the actual value.
        n @ 1..=10 => format!("small: {}", n),

        // NOTE: Same pattern for negative range.
        // -10..=-1 is a range of negative numbers, inclusive on both ends.
        n @ -10..=-1 => format!("neg small: {}", n),

        // NOTE: Match guard — `n if condition` adds a runtime condition on top of the pattern.
        // This arm catches everything not matched above.
        // We don't need @ here because `n` in the pattern already binds the value.
        n => format!("big: {}", n),
    }
}

fn main() {
    println!("{}", classify(0));    // zero
    println!("{}", classify(7));    // small: 7
    println!("{}", classify(-5));   // neg small: -5
    println!("{}", classify(100));  // big: 100
    println!("{}", classify(-99));  // big: -99
}
