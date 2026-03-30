// Code Generation with Macros
// Run: cargo run --bin macros

// NOTE: `macro_rules!` defines a pattern-based macro.
// `$x:expr` is a metavariable — `:expr` means it accepts any Rust expression.
// The `=>` arm expands the pattern into code at compile time, not runtime.
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn compute(n: i32) -> i32 {
    // NOTE: square!(n) is replaced by `n * n` by the compiler before codegen.
    // No function call overhead — it's a textual expansion.
    square!(n)
}

fn main() {
    println!("{}", compute(4));    // 16
    println!("{}", compute(-3));   // 9
    println!("{}", square!(2 + 1)); // NOTE: expands to (2+1)*(2+1) = 9, not 2+1*2+1
}
