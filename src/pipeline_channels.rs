// Composing Concurrent Stages with Channel Pipelines
// Run: cargo run --bin pipeline_channels

use std::sync::mpsc;
use std::thread;

fn pipeline(input: Vec<i32>) -> Vec<String> {
    // --- Stage 1: filter even numbers ---
    // NOTE: Each stage gets its own (tx, rx) pair.
    // Dropping tx at the end of the closure closes the channel, signalling the next stage to stop.
    let (tx1, rx1) = mpsc::channel::<i32>();
    thread::spawn(move || {
        for n in input {
            if n % 2 == 0 {
                tx1.send(n).unwrap();
            }
        }
        // NOTE: tx1 drops here → rx1.iter() in stage 2 terminates.
    });

    // --- Stage 2: square ---
    let (tx2, rx2) = mpsc::channel::<i32>();
    thread::spawn(move || {
        // NOTE: rx1.iter() blocks until a value arrives, returns None when tx1 is dropped.
        for n in rx1.iter() {
            tx2.send(n * n).unwrap();
        }
        // NOTE: tx2 drops here → rx2.iter() in stage 3 terminates.
    });

    // --- Stage 3: convert to String --- (runs on the calling thread, no extra spawn needed)
    // NOTE: The final stage collects directly; no need to spawn a thread just to format.
    rx2.iter().map(|n| n.to_string()).collect()
}

fn main() {
    println!("{:?}", pipeline(vec![1, 2, 3, 4, 5, 6])); // ["4", "16", "36"]
    println!("{:?}", pipeline(vec![1, 3, 5]));           // []
    println!("{:?}", pipeline(vec![]));                  // []
}
