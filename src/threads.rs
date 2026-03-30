// Fearless Concurrency with Threads
// Run: cargo run --bin threads

use std::thread;

fn parallel_sum(nums: Vec<i32>) -> i32 {
    // NOTE: split_at requires an index; mid gives us two equal (or near-equal) halves.
    let mid = nums.len() / 2;

    // NOTE: split_at returns two &[i32] slices — we need owned data to move into threads.
    // to_vec() clones each half so each thread owns its chunk independently.
    let (left, right) = nums.split_at(mid);
    let left = left.to_vec();
    let right = right.to_vec();

    // NOTE: `move` transfers ownership of `left` into the thread.
    // The JoinHandle<i32> carries the return value of the closure.
    let h1 = thread::spawn(move || left.iter().sum::<i32>());
    let h2 = thread::spawn(move || right.iter().sum::<i32>());

    // NOTE: .join() blocks until the thread finishes and returns Result<T, _>.
    // .unwrap() is fine here — the closure can't panic on a simple sum.
    h1.join().unwrap() + h2.join().unwrap()
}

fn main() {
    println!("{}", parallel_sum(vec![1, 2, 3, 4, 5, 6])); // 21
    println!("{}", parallel_sum(vec![10, 20]));            // 30
    println!("{}", parallel_sum(vec![]));                  // 0
}
