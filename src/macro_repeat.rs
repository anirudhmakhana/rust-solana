// Macro Repetitions — Variable-Length Arguments
// Run: cargo run --bin macro_repeat

// NOTE: `$( $x:expr ),*` is a repetition pattern.
// `$( ... ),*` means: match zero or more occurrences, separated by commas.
// In the expansion, `$( ... )*` repeats the body once per matched element.
macro_rules! sum {
    // NOTE: Empty arm — must come first so `sum!()` matches before the repetition arm.
    () => { 0 };

    // NOTE: The expansion uses `0 $( + $x )*` — starts with 0 then adds each element.
    // This handles both the empty-after-zero case cleanly without a base-case recursion.
    ( $( $x:expr ),* ) => {
        0 $( + $x )*
    };
}

fn total(a: i32, b: i32, c: i32) -> i32 {
    sum!(a, b, c)
}

fn main() {
    println!("{}", sum!());           // 0
    println!("{}", sum!(1));          // 1
    println!("{}", sum!(1, 2, 3));    // 6
    println!("{}", total(10, 20, 30)); // 60
}
