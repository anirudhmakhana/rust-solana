// NOTE: Regular named functions automatically have the type fn(i32) -> i32.
// Unlike closures (|x| x * 2), function pointers don't capture any environment.
// They are just a pointer to a location in the compiled binary.
fn double(x: i32) -> i32 {
    x * 2
}

fn increment(x: i32) -> i32 {
    x + 1
}

// NOTE: `f: fn(i32) -> i32` — f is a function pointer, not a closure.
// fn(...) -> ... is a concrete type, not a trait like Fn/FnMut/FnOnce.
// You can store it, pass it, call it — but it cannot capture variables from its surroundings.
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    // NOTE: First apply f to x, then apply f to that result.
    // apply_twice(double, 3) → double(double(3)) → double(6) → 12
    f(f(x))
}

fn main() {
    // NOTE: double and increment are passed by value as function pointers.
    // No & needed — fn pointers are Copy (they're just an address, 8 bytes).
    println!("{}", apply_twice(double, 3));     // 12
    println!("{}", apply_twice(increment, 10)); // 12

    // NOTE: You can also store function pointers in a variable.
    let op: fn(i32) -> i32 = double;
    println!("{}", apply_twice(op, 5)); // 20
}
