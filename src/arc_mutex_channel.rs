// Arc<Mutex> Counter — single thread with channel result
// Run: cargo run --bin arc_mutex_channel

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn concurrent_counter(n: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0_i32));
    let c = Arc::clone(&counter);
    let (tx, rx) = mpsc::channel();

    // NOTE: Single thread does all n increments, then sends the final value back via channel.
    // No need to join + re-lock — the send carries the result directly to the caller.
    thread::spawn(move || {
        for _ in 0..n {
            *c.lock().unwrap() += 1;
        }
        tx.send(*c.lock().unwrap()).unwrap();
    });

    rx.recv().unwrap()
}

fn main() {
    println!("{}", concurrent_counter(5));   // 5
    println!("{}", concurrent_counter(100)); // 100
    println!("{}", concurrent_counter(0));   // 0
}
