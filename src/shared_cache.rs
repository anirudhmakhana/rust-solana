// Shared Cache Pattern with Arc<Mutex<HashMap>>
// Run: cargo run --bin shared_cache

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn cached_squares(inputs: Vec<i32>) -> Vec<i32> {
    let cache: Arc<Mutex<HashMap<i32, i32>>> = Arc::new(Mutex::new(HashMap::new()));

    // NOTE: Dedup so we only spawn one thread per unique value — no redundant work.
    let mut unique: Vec<i32> = inputs.clone();
    unique.sort_unstable();
    unique.dedup();

    let handles: Vec<_> = unique
        .into_iter()
        .map(|x| {
            let c = Arc::clone(&cache);
            thread::spawn(move || {
                // NOTE: Lock, insert, and immediately drop the guard by ending the block.
                // Holding the lock across a computation blocks all other threads unnecessarily.
                c.lock().unwrap().insert(x, x * x);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    // NOTE: All threads done — single lock acquisition to read all results in order.
    let cache = cache.lock().unwrap();
    inputs.iter().map(|x| *cache.get(x).unwrap()).collect()
}

fn main() {
    println!("{:?}", cached_squares(vec![1, 2, 3, 2, 1])); // [1, 4, 9, 4, 1]
    println!("{:?}", cached_squares(vec![5, 5, 5]));       // [25, 25, 25]
    println!("{:?}", cached_squares(vec![]));              // []
}
