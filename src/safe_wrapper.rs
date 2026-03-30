// The Safe Wrapper Pattern
// Run: cargo run --bin safe_wrapper

struct SafeArray {
    data: Vec<i32>,
}

impl SafeArray {
    fn new(data: Vec<i32>) -> Self {
        SafeArray { data }
    }

    // NOTE: Safe layer — bounds check first, return None if out of range.
    // Callers never need to worry about invalid indices.
    fn get(&self, i: usize) -> Option<i32> {
        if i < self.data.len() {
            // NOTE: We've already verified the index is valid, so get_unchecked is sound here.
            Some(unsafe { self.get_unchecked(i) })
        } else {
            None
        }
    }

    // NOTE: Marked `unsafe` because the caller must guarantee `i < self.data.len()`.
    // Using an out-of-bounds index is undefined behaviour — that contract is on the caller.
    unsafe fn get_unchecked(&self, i: usize) -> i32 {
        // NOTE: Rust 2024 requires an explicit unsafe block even inside an unsafe fn.
        // as_ptr() gives a raw *const i32; .add(i) advances by i elements (pointer arithmetic).
        unsafe { *self.data.as_ptr().add(i) }
    }

    // NOTE: Safe public API — iterates over known-valid indices, so the unsafe call inside is sound.
    // This is the pattern the stdlib uses: verify once at the boundary, use unchecked inside.
    fn sum_all(&self) -> i32 {
        let mut total = 0;
        for i in 0..self.data.len() {
            total += unsafe { self.get_unchecked(i) };
        }
        total
    }
}

fn main() {
    let arr = SafeArray::new(vec![1, 2, 3, 4, 5]);

    println!("{:?}", arr.get(2));  // Some(3)
    println!("{:?}", arr.get(9));  // None
    println!("{}", arr.sum_all()); // 15
}
