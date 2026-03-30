// Single-Threaded Counter with Mutex (no Arc needed)
// Run: cargo run --bin single_thread_counter

use std::sync::Mutex;

// NOTE: No Arc here — single thread means no shared ownership.
// Mutex alone is sufficient; we own it directly on the stack.
fn concurrent_counter(n: usize) -> i32 {
    let counter = Mutex::new(0_i32);
    for _ in 0..n {
        // NOTE: Same lock/unlock pattern as the multi-threaded Arc<Mutex> version.
        // MutexGuard drops at end of the statement, releasing the lock immediately.
        *counter.lock().unwrap() += 1;
    }
    *counter.lock().unwrap()
}

fn main() {
    println!("{}", concurrent_counter(5));   // 5
    println!("{}", concurrent_counter(100)); // 100
    println!("{}", concurrent_counter(0));   // 0
}
