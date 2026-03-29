// NOTE: Without Box here, this would be a compile error.
// The compiler needs to know the size of List at compile time.
// Cons(i32, List) would mean List contains itself — infinite size, impossible.
// Box<List> fixes this: Box is always pointer-sized (8 bytes on 64-bit), size known.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// NOTE: We take &List (borrow) not List (ownership) so the caller keeps the list.
// If we took List by value, it would be moved in and dropped after the call.
fn list_sum(list: &List) -> i32 {
    match list {
        // NOTE: Pattern match destructures the Cons variant into its two fields.
        // `val` is &i32 (auto-ref from matching on &List), `rest` is &Box<List>.
        // Rust auto-derefs &Box<List> to &List when we call list_sum(rest) — no * needed.
        List::Cons(val, rest) => val + list_sum(rest),

        // NOTE: Base case. An empty list contributes 0 to the sum.
        // match is exhaustive — the compiler forces us to handle Nil.
        List::Nil => 0,
    }
}

fn main() {
    // NOTE: Building the list 3 -> 2 -> 1 -> Nil.
    // Each Cons takes ownership of a Box — the Box allocates the inner List on the heap.
    // Box::new(x) moves x to the heap and returns a Box<T> pointer to it.
    let list = List::Cons(
        3,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(
                1,
                // NOTE: Nil marks the end of the list — like null in a linked list,
                // but safe: the type system guarantees we always handle it.
                Box::new(List::Nil),
            )),
        )),
    );

    // NOTE: We pass &list — borrow, not move.
    // After this line, list is still valid and owned by main.
    println!("{}", list_sum(&list)); // 6
}
