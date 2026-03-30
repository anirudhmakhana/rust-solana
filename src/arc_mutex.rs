// Shared Mutable State with Arc<Mutex<T>>
// Run: cargo run --bin arc_mutex

use std::sync::{Arc, Mutex};
use std::thread;

fn concurrent_counter(n: usize) -> i32 {
    // NOTE: Mutex<i32> protects the counter — only one thread can hold the lock at a time.
    // Arc (Atomic Reference Count) lets multiple threads share ownership of the Mutex.
    let counter = Arc::new(Mutex::new(0_i32));

    let handles: Vec<_> = (0..n)
        .map(|_| {
            // NOTE: Arc::clone increments the reference count — cheap, doesn't clone the data.
            // Each thread gets its own Arc handle pointing to the same Mutex.
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                // NOTE: .lock() blocks until the mutex is free, then returns a MutexGuard.
                // The guard auto-unlocks when it drops at the end of this block.
                *c.lock().unwrap() += 1;
            })
        })
        .collect();

    // NOTE: join each handle before reading the result — otherwise some threads
    // may not have finished incrementing yet.
    for h in handles {
        h.join().unwrap();
    }

    // NOTE: All threads are done; no contention here. Unwrap is safe.
    *counter.lock().unwrap()
}

fn main() {
    println!("{}", concurrent_counter(5));   // 5
    println!("{}", concurrent_counter(100)); // 100
    println!("{}", concurrent_counter(0));   // 0
}
