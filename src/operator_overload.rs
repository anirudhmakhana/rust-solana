use std::ops::Add;
use std::fmt;

struct Vec2 {
    x: f64,
    y: f64,
}

// NOTE: Add is a trait from std::ops. Implementing it defines what `a + b` means for Vec2.
// `type Output = Vec2` — the result of adding two Vec2s is another Vec2.
// `add(self, rhs)` takes OWNERSHIP of both sides — a + b moves a and b.
// If you want to reuse a or b after addition, you'd need to impl Add for &Vec2 instead.
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// NOTE: Display controls how Vec2 prints with {}.
// {:.1} formats a float to exactly 1 decimal place.
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

fn add_vecs(a: Vec2, b: Vec2) -> String {
    // NOTE: a + b calls our Add impl — desugars to Add::add(a, b).
    // The result is a Vec2, then format! calls its Display impl to produce a String.
    format!("{}", a + b)
}

fn main() {
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.5, y: 4.5 };
    println!("{}", add_vecs(a, b)); // (4.5, 6.5)
}
