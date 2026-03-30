// Pattern-Based Code Generation with Multi-Arm Macros
// Run: cargo run --bin macro_arms

// NOTE: Each arm matches a different *syntactic* pattern, not a runtime value.
// The compiler picks the arm at compile time based on the tokens you pass in.
macro_rules! convert {
    (celsius_to_f, $temp:expr) => {
        $temp * 9 / 5 + 32
    };
    (f_to_celsius, $temp:expr) => {
        ($temp - 32) * 5 / 9
    };
}

fn temp_test(c: i32) -> i32 {
    convert!(celsius_to_f, c)
}

fn main() {
    println!("{}", temp_test(0));                    // 32
    println!("{}", temp_test(100));                  // 212
    println!("{}", convert!(f_to_celsius, 212));     // 100
    println!("{}", convert!(f_to_celsius, 32));      // 0
}
