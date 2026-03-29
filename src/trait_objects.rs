// NOTE: `dyn Trait` = dynamic dispatch. The concrete type is NOT known at compile time.
// Rust uses a vtable (virtual dispatch table) to look up the right method at runtime.
// Contrast with generics (<T: Shape>) which resolve at compile time (static dispatch).
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rect {
    w: f64,
    h: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        // NOTE: std::f64::consts::PI is the standard library constant for π.
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

// NOTE: &[Box<dyn Shape>] — a slice of heap-allocated trait objects.
// Each Box<dyn Shape> holds:
//   1. A pointer to the concrete data on the heap (Circle or Rect)
//   2. A pointer to the vtable for that concrete type
// We don't know at compile time whether each element is a Circle or Rect — that's the point.
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    // NOTE: Iterator pipeline — borrow each Box, call .area() via vtable, sum the results.
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    // NOTE: The Vec holds different concrete types behind Box<dyn Shape>.
    // Without Box, the compiler can't build a Vec<dyn Shape> — each element would be a different size.
    // Box gives them all the same size: one pointer (8 bytes).
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rect { w: 2.0, h: 3.0 }),
        Box::new(Circle { radius: 2.0 }),
    ];

    println!("{:.4}", total_area(&shapes));
}
