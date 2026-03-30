use std::fmt;

// NOTE: The newtype pattern — a tuple struct wrapping one existing type.
// This makes CommaSeparated "our type", so we can implement foreign traits (like Display) on it.
// Orphan rule: you cannot impl a foreign trait (Display) for a foreign type (Vec<i32>).
// Wrapping Vec<i32> in our own struct bypasses this — CommaSeparated belongs to us.
struct CommaSeparated(Vec<i32>);

// NOTE: Display is the trait that powers {} in println!/format!.
// Implementing it requires writing to a fmt::Formatter — a buffer that collects the output.
impl fmt::Display for CommaSeparated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: self.0 accesses the first (only) field of the tuple struct — the Vec<i32>.
        // We iterate with enumerate() to know when we're at the last element (no trailing comma).
        let len = self.0.len();
        for (i, val) in self.0.iter().enumerate() {
            if i < len - 1 {
                // NOTE: write! is like print! but targets the formatter buffer instead of stdout.
                // The ? propagates any formatting error upward (same ? operator from Result).
                write!(f, "{}, ", val)?;
            } else {
                write!(f, "{}", val)?;
            }
        }
        Ok(())
    }
}

fn format_list(nums: Vec<i32>) -> String {
    // NOTE: CommaSeparated takes ownership of nums (moved in).
    // format!("{}", x) calls x's Display impl — our fmt() above.
    // This returns an owned String.
    format!("{}", CommaSeparated(nums))
}

fn main() {
    println!("{}", format_list(vec![1, 2, 3]));       // 1, 2, 3
    println!("{}", format_list(vec![10, 20, 30, 40])); // 10, 20, 30, 40
    println!("{}", format_list(vec![42]));              // 42
}
