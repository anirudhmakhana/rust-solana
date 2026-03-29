use std::rc::Rc;

fn count_owners(n: usize) -> usize {
    // NOTE: Rc::new moves the value to the heap and wraps it in a reference counter.
    // The count starts at 1 — this binding is the first owner.
    let shared = Rc::new("shared".to_string());

    // NOTE: Rc::clone does NOT deep-copy the data.
    // It only increments the reference count and returns a new Rc pointing to the SAME heap allocation.
    // This is O(1) — cheap, regardless of the size of the data inside.
    let _clones: Vec<Rc<String>> = (0..n).map(|_| Rc::clone(&shared)).collect();

    // NOTE: strong_count returns the number of Rc pointers to this allocation.
    // Here: 1 (shared) + n (clones) = n + 1.
    // When all Rc's go out of scope, count hits 0 and the heap data is freed.
    Rc::strong_count(&shared)
}

fn main() {
    println!("{}", count_owners(0)); // 1
    println!("{}", count_owners(3)); // 4
    println!("{}", count_owners(5)); // 6
}
