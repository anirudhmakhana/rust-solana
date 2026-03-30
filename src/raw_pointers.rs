// Raw pointers and unsafe Rust
// Run: cargo run --bin raw_pointers

fn swap_values(a: &mut i32, b: &mut i32) {
    // NOTE: Cast &mut to *mut — this is safe to create, unsafe to dereference.
    let pa: *mut i32 = a;
    let pb: *mut i32 = b;

    // NOTE: unsafe block is required to dereference raw pointers.
    // ptr::swap handles the actual swap atomically without a temp variable.
    unsafe { std::ptr::swap(pa, pb); }
}

fn main() {
    let mut x = 10;
    let mut y = 20;
    swap_values(&mut x, &mut y);
    println!("x={}, y={}", x, y); // x=20, y=10
}
