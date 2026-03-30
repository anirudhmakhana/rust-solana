// NOTE: We cannot write `fn make_multiplier(n: i32) -> impl Fn(i32) -> i32` with compose,
// because each closure has a unique anonymous type — you can't name it, store it, or pass it
// as a generic across function boundaries easily. Box<dyn Fn(...)> solves this:
// it heap-allocates the closure and gives us a single, uniform pointer type.

fn make_multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
    // NOTE: `move` captures n by VALUE into the closure.
    // Without `move`, n would be a reference — but n is a local variable on the stack,
    // and it would be gone after this function returns. `move` transfers ownership into the closure.
    Box::new(move |x| x * n)
}

fn compose(
    f: Box<dyn Fn(i32) -> i32>,
    g: Box<dyn Fn(i32) -> i32>,
) -> Box<dyn Fn(i32) -> i32> {
    // NOTE: `move` captures f and g by value — moves the two Boxes into the new closure.
    // The returned closure owns f and g; they live on the heap for as long as the closure does.
    // f(g(x)) = apply g first, feed result into f.
    Box::new(move |x| f(g(x)))
}

fn main() {
    let double = make_multiplier(2);
    let triple = make_multiplier(3);

    println!("{}", double(5));  // 10
    println!("{}", triple(5));  // 15

    // NOTE: compose(double, triple) creates a new closure: x → double(triple(x))
    // triple(4) = 12, double(12) = 24
    let sextuple = compose(make_multiplier(2), make_multiplier(3));
    println!("{}", sextuple(4)); // 24
}
