// Message Passing Concurrency with Channels
// Run: cargo run --bin channels

use std::sync::mpsc;
use std::thread;

fn process_values(values: Vec<i32>) -> Vec<i32> {
    // NOTE: mpsc = multi-producer, single-consumer.
    // tx (transmitter) is moved into the thread; rx (receiver) stays on this thread.
    let (tx, rx) = mpsc::channel();

    // NOTE: `move` captures `values` and `tx` by value into the new thread.
    // Without `move`, the borrow would dangle — the spawned thread may outlive the current scope.
    thread::spawn(move || {
        for v in values {
            // NOTE: send() returns Err if the receiver has been dropped.
            // .unwrap() is fine here — rx is alive in the outer scope.
            tx.send(v * 2).unwrap();
        }
        // NOTE: tx is dropped here when the closure ends, which closes the channel
        // and causes rx iteration below to stop.
    });

    // NOTE: rx is an iterator — it blocks on each .next() until a value arrives,
    // and returns None when the channel is closed (tx dropped).
    rx.iter().collect()
}

fn main() {
    println!("{:?}", process_values(vec![1, 2, 3, 4, 5])); // [2, 4, 6, 8, 10]
    println!("{:?}", process_values(vec![]));               // []
}
