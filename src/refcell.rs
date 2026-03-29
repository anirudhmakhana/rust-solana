use std::cell::RefCell;

struct Counter {
    // NOTE: RefCell<i32> wraps an i32 with runtime borrow checking.
    // The field itself is not mut — that's the whole point of RefCell.
    // Interior mutability: the outer struct looks immutable, but the inner value can be changed.
    value: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }

    // NOTE: &self — immutable reference to Counter.
    // Normally this would mean we cannot mutate anything inside.
    // RefCell lets us bypass this — borrow_mut() checks the rule at RUNTIME instead.
    // If two borrow_mut() calls overlap, it panics (instead of a compile error like &mut would give).
    fn increment(&self) {
        *self.value.borrow_mut() += 1;
        // NOTE: borrow_mut() returns a RefMut<i32> — a smart pointer to the inner value.
        // The * dereferences it so we can do += 1 on the actual i32.
        // RefMut is dropped at the end of this statement, releasing the borrow.
    }

    // NOTE: borrow() returns a Ref<i32> — an immutable smart pointer to the inner value.
    // The * dereferences it to get the i32 value.
    // Borrow rule still applies at runtime: you can have many borrow() OR one borrow_mut(), not both.
    fn get(&self) -> i32 {
        *self.value.borrow()
    }
}

fn count_to(n: i32) -> i32 {
    let counter = Counter::new();
    for _ in 0..n {
        counter.increment();
    }
    counter.get()
}

fn main() {
    println!("{}", count_to(0));  // 0
    println!("{}", count_to(5));  // 5
    println!("{}", count_to(10)); // 10
}
