# Complete Exam Review

Full coverage of every concept tested across all 6 sections.
Written after your exam — includes the ones you got right, the ones you guessed, and the ones you missed.
Don't skim the sections you got right. If you guessed correctly, the explanation is what matters.

---

## Your Exam Scorecard

| Section | Score | Topics |
|---|---|---|
| 1 — Stack, Heap & References | 3.5 / 5 | Memory, &, * |
| 2 — Variables, Types & Control Flow | 5 / 6 | let, mut, shadowing, if, match, ranges |
| 3 — Ownership & Borrowing | 3 / 7 | move, borrow, &mut, functions |
| 4 — Structs, Enums & Traits | 5 / 6 | impl, self, derive, enums, nominal typing |
| 5 — Error Handling | 3 / 5 | Option, Result, unwrap, ? |
| 6 — Iterators & Closures | 4 / 5 | lazy eval, adapters, consumers, closures |
| **Total** | **23.5 / 34** | |

---

# Section 1 — Stack, Heap & References

---

## Stack vs Heap — Where Values Live

Every value in your program lives in one of two places. Knowing which is which explains almost everything about Rust.

### The Stack

Think of it as a stack of trays in a cafeteria. You add a tray on top, use it, then remove it. It's fast because:
- The size of everything on it is **known at compile time**
- Adding and removing is just moving a pointer — no searching, no bookkeeping

When a function is called, a **stack frame** is pushed — all the local variables live there.
When the function returns, the frame is popped — all those variables vanish instantly and automatically.

```
fn main() is called → frame pushed
  fn foo() is called → frame pushed
    fn bar() is called → frame pushed
    bar() returns → frame popped
  foo() returns → frame popped
main() returns → frame popped
```

Types that live entirely on the stack:
```
i8, i16, i32, i64, i128, isize
u8, u16, u32, u64, u128, usize
f32, f64
bool
char
(i32, f64)   — tuple where all fields are stack types
[i32; 4]     — fixed-size array where elements are stack types
&T           — a reference IS just an address (a number) — lives on the stack
```

### The Heap

Think of it as a storage warehouse. You can store things of any size, including sizes you don't know until the program runs. But:
- You have to **ask** for space (allocate)
- Someone has to **give it back** when done (deallocate)
- Finding free space takes real work compared to just moving a pointer

In C you did this manually with `malloc` and `free`. Forget one `free` → memory leak. Free twice → crash.
In Rust, the **ownership system** does this automatically — deterministically, with zero overhead.

Types that use heap memory:
```
String          — the text data lives on the heap
Vec<T>          — the elements live on the heap
HashMap<K, V>   — the key-value pairs live on the heap
Box<T>          — explicitly heap-allocated value
```

### What String actually looks like in memory

```
Stack                      Heap
┌─────────────────┐       ┌───┬───┬───┬───┬───┐
│ ptr  ────────────────►  │ h │ e │ l │ l │ o │
│ len: 5          │       └───┴───┴───┴───┴───┘
│ cap: 5          │
└─────────────────┘
```

The **stack part** (pointer, length, capacity) is small and fixed-size — 24 bytes on a 64-bit machine.
The **heap part** is the actual text — its size depends on the string content.

This is why `String` can't be `Copy` — copying it would mean two stack parts pointing to the same heap data. When one drops, the heap gets freed. The other now points to freed memory. Disaster.

---

## What `&` Creates — References

### The core idea

`&` creates a **reference** — a value that holds the memory address of another value.
Instead of giving someone the thing itself, you give them a slip of paper with the address of where the thing lives.

```rust
let x = 42;
let r = &x;         // r holds the address of x, not a copy of 42

println!("{}", x);  // prints 42 — x unchanged
println!("{}", r);  // prints 42 — Rust automatically follows r to get the value
println!("{}", *r); // prints 42 — * manually follows the reference ("dereference")
```

The `*` operator is called **dereference** — it follows the reference arrow and gives you what's at the other end.

### Why references exist

Without references, the only way to pass a value to a function would be to either:
1. **Move** it into the function (caller loses it forever)
2. **Copy** it (only works for `Copy` types — expensive for large data)

References give you a third option: **lend** it. The function can use it, then the owner gets it back.

```rust
let name = String::from("Alice");

// Without references: move
fn greet(s: String) { println!("Hello {}", s); }
greet(name);
// name is gone — moved into greet

// With references: borrow
fn greet(s: &str) { println!("Hello {}", s); }
greet(&name);
// name still valid — we only lent it
```

### The two kinds of reference

```rust
&T      // immutable reference — read-only access, no changes allowed
&mut T  // mutable reference — read AND write access
```

### Where you'll see & constantly

```rust
fn foo(x: &i32)        // parameter: borrow an i32
fn foo(x: &mut Vec<i32>) // parameter: mutably borrow a Vec
let r = &value;        // create a reference to value
let s: &str = "hello"; // &str is a reference to string data
&v[0]                  // reference to the first element of v
```

---

## Drop — What Happens When an Owner Goes Out of Scope

When a variable goes out of scope (hits the closing `}`), Rust automatically calls a special function called `drop`. For heap types, `drop` frees the heap memory.

```rust
{
    let s = String::from("hello");  // heap memory allocated
    // ... use s ...
}   // ← s goes out of scope here. drop(s) is called. Heap memory freed.

// No garbage collector. No manual free(). Deterministic. Zero overhead.
```

This pattern — "resource is freed when owner goes out of scope" — is called **RAII** (Resource Acquisition Is Initialization). It's not just memory: file handles, network connections, locks — anything that needs cleanup uses this pattern.

---

# Section 2 — Variables, Types & Control Flow

---

## Immutability by Default

```rust
let x = 10;
x = 20;     // ❌ compile error: cannot assign twice to immutable variable
```

This is not a runtime crash — it's caught before the program runs. The compiler refuses to produce a binary that could accidentally mutate `x`.

To allow reassignment:
```rust
let mut x = 10;
x = 20;     // ✅
```

### Why immutable by default?

Most variables in real programs don't need to change after creation. Making mutation opt-in (`mut`) forces you to be explicit about what changes and what doesn't — making code easier to reason about.

---

## Shadowing vs `mut` — The Full Picture

These look similar but are fundamentally different:

### `mut` — reassign the same binding

```rust
let mut x = 5;
x = 10;         // reassigns x — same variable, new value
x = "hello";    // ❌ compile error — can't change the TYPE with mut
```

`mut` lets you change the **value** but the **type is locked** at declaration.

### Shadowing — create a brand new binding

```rust
let x = 5;
let x = 10;         // new variable called x — old x is gone
let x = "hello";    // new variable called x — can change type!
println!("{}", x);  // prints "hello"
```

Each `let x = ...` creates a completely new variable that happens to reuse the name.
The old variable is discarded (or rather, hidden beneath the new one).

### When each is the right tool

```
mut      → the value genuinely changes over time (loop counter, accumulator)
shadowing → transforming a value through steps (parse a string → trim → convert type)
```

Real-world shadowing pattern from the source files:
```rust
let input = "  42  ";       // &str — raw user input
let input = input.trim();   // &str — trimmed (same type, different value)
let input: i32 = input.parse().unwrap(); // i32 — converted to number
// Much cleaner than: input_raw, input_trimmed, input_parsed
```

---

## If as Expression

In most languages, `if` is a **statement** — it does something but doesn't produce a value.
In Rust, `if` is an **expression** — it produces a value that you can use.

```rust
// if as a statement (what you know from other languages)
let grade;
if score >= 90 {
    grade = "A";
} else {
    grade = "B";
}

// if as an expression (the Rust way)
let grade = if score >= 90 { "A" } else { "B" };
```

Both branches **must return the same type**. The compiler rejects this:
```rust
let x = if condition { 5 } else { "hello" }; // ❌ i32 vs &str — different types
```

A lone `if` without `else` has type `()` (the empty type, pronounced "unit"):
```rust
if condition {
    println!("yes");
}
// This is fine — it's an expression that produces ()
```

---

## Ranges — `..` vs `..=`

```rust
1..5    // exclusive — produces: 1, 2, 3, 4      (stops BEFORE 5)
1..=5   // inclusive — produces: 1, 2, 3, 4, 5   (includes 5)
```

The `=` in `..=` means "equals the end value too."

```rust
for i in 1..5  { print!("{} ", i); }  // 1 2 3 4
for i in 1..=5 { print!("{} ", i); }  // 1 2 3 4 5
```

Range patterns in match:
```rust
match score {
    90..=100 => "A",   // 90, 91, 92 ... 100
    80..=89  => "B",
    _        => "F",
}
```

---

## Match — Exhaustive Pattern Matching

`match` is like a supercharged `switch` statement — but it must cover every possible case.

```rust
let coin = "dime";
let value = match coin {
    "penny"   => 1,
    "nickel"  => 5,
    "dime"    => 10,
    "quarter" => 25,
    _         => 0,    // wildcard: catches anything not listed above
};
// value = 10
```

Key properties:
- `match` is an **expression** — it produces a value
- All arms must return the **same type**
- It is **exhaustive** — missing a case is a compile error
- `_` is the wildcard catch-all

Without `_`, the compiler forces you to handle every possible value:
```rust
enum Direction { North, South, East, West }

match dir {
    Direction::North => "up",
    Direction::South => "down",
    // ❌ compile error: non-exhaustive patterns — East and West not covered
}
```

This is intentional — if you add `Direction::Up` later, every match site breaks at compile time, forcing you to handle the new case.

---

# Section 3 — Ownership & Borrowing

This section had the most missed questions (3/7). Read everything here.

---

## The Three Ownership Rules

```
1. Every value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.
```

---

## Copy vs Move — The Fundamental Split

Every type is either **Copy** or **non-Copy**. This decides what happens on assignment.

| Type is... | Assignment does... | Original after |
|---|---|---|
| `Copy` | Silently duplicates the bits | Still valid |
| Non-Copy | Transfers ownership (moves) | Gone — compile error if used |

### Copy types (stack-only, trivially duplicatable)

```
i8, i16, i32, i64, i128, isize
u8, u16, u32, u64, u128, usize
f32, f64
bool
char
(i32, f64)   — tuple where ALL fields are Copy
[i32; 4]     — array where the element type is Copy
&T           — a reference itself is Copy (the address, not the data)
```

### Non-Copy types (own heap memory)

```
String, Vec<T>, HashMap<K,V>, Box<T>
Any struct you define (unless you #[derive(Copy, Clone)])
```

### In action

```rust
// i32 is Copy
let a: i32 = 5;
let b = a;           // a is COPIED into b
println!("{}", a);   // ✅ a still exists — it was copied
println!("{}", b);   // ✅

// String is NOT Copy
let s1 = String::from("hello");
let s2 = s1;              // s1 is MOVED into s2
println!("{}", s1);       // ❌ compile error: s1 was moved
println!("{}", s2);       // ✅

// If you want both to exist: clone
let s1 = String::from("hello");
let s2 = s1.clone();      // deep copy — both are independent
println!("{}", s1);       // ✅
println!("{}", s2);       // ✅
```

### The mental model

> "Does this type own heap memory?"
> No → probably `Copy`. Assignment duplicates.
> Yes → `Move`. Assignment transfers ownership.

---

## Immutable Borrows — Unlimited Readers

You can have **as many `&T` references as you want**, all at the same time.

```rust
let s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &s;  // still fine — no limit on immutable borrows

println!("{} {} {}", r1, r2, r3);  // ✅ all valid
```

Why is this safe? None of them can modify the data. If nobody writes, everyone can read simultaneously with zero risk — like a read-only Google Doc with 100 viewers.

---

## Mutable Borrows — Exclusive Writers

You can have **exactly one `&mut T` reference** at a time — and while it exists, **no other references (mutable or immutable) can exist**.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // ❌ compile error: cannot borrow s as mutable more than once
```

```rust
let mut s = String::from("hello");
let r1 = &s;       // immutable borrow
let r2 = &mut s;   // ❌ compile error: cannot borrow as mutable because already borrowed as immutable
println!("{} {}", r1, r2);
```

### The complete borrowing rule

```
At any moment, you can have EITHER:
  ✅ Any number of &T (immutable references)
OR
  ✅ Exactly one &mut T (mutable reference)

NEVER both at the same time.
```

### Why this rule exists

Imagine the compiler allowed `&T` and `&mut T` at the same time:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // "I promise the first element stays at this address"
v.push(100);         // might reallocate v to a bigger buffer, free the old one
println!("{}", first); // first now points to freed memory — undefined behavior
```

Rust catches this at compile time. No segfault, no crash, no undefined behavior.

### NLL — when exactly does a borrow end?

With modern Rust (NLL — Non-Lexical Lifetimes), a borrow ends at its **last use**, not at the closing `}`:

```rust
let mut s = String::from("hello");
let r = &s;
println!("{}", r);   // ← r's last use — borrow ends HERE
                     // (not at the end of the function)
s.push_str("!");     // ✅ fine — r is no longer active
```

This means you can often sequence borrows without extra scopes.

---

## Move in Function Calls

Passing a value to a function follows the same rules as assignment:

```rust
fn takes_ownership(s: String) {  // s: String means "I take ownership"
    println!("{}", s);
}   // s drops here — heap memory freed

let my_string = String::from("hello");
takes_ownership(my_string);          // my_string MOVED into the function
println!("{}", my_string);           // ❌ compile error — my_string was moved
```

```rust
fn borrows(s: &str) {  // &str means "I borrow a view of the string"
    println!("{}", s);
}   // borrow ends — nothing freed

let my_string = String::from("hello");
borrows(&my_string);                 // my_string BORROWED by the function
println!("{}", my_string);           // ✅ my_string still valid
```

### The four ways to pass a value

```rust
fn f(s: String)       // MOVE: caller loses s permanently
fn f(s: &String)      // BORROW: caller keeps s, function reads it
fn f(s: &str)         // BORROW: most flexible for strings — prefer this
fn f(s: &mut String)  // MUT BORROW: caller keeps s, function can modify it
```

### When to move into a function

When the function needs to **store** the value long-term:

```rust
struct User { name: String }

impl User {
    fn new(name: String) -> Self {  // takes ownership to store it
        User { name }
    }
}

let n = String::from("Alice");
let user = User::new(n);   // n is moved — user.name now owns it
```

---

## Vec, References, and Reallocation

This is Q18 — the trickiest question in the exam.

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // borrow: first points into v's heap buffer
v.push(4);           // ❌ compile error
println!("{}", first);
```

Why does this fail? A `Vec` has a **capacity**. When you push beyond capacity:
1. A new, bigger buffer is allocated elsewhere in memory
2. All elements are copied to the new buffer
3. The old buffer is **freed**

```
Before push:
Stack: ptr ──────► [1][2][3]   (cap=3, full)
       first ────► [1] (same address)

After push (with reallocation):
Stack: ptr ──────────────────► [1][2][3][4]   (new location, cap=6)
       first ─────► [?][?][?]  ← freed memory! dangling pointer!
```

`first` would point to freed memory. Rust refuses to compile it.

### Three ways to fix it

```rust
// Fix 1: Copy the i32 out (no reference held)
let first = v[0];    // i32 is Copy — copies the value, no borrow
v.push(4);           // ✅ no active borrow on v

// Fix 2: Mutate first, borrow after
v.push(4);           // ✅ mutate first
let first = &v[0];   // borrow after — safe

// Fix 3: End the borrow before mutating (NLL)
let first = &v[0];
println!("{}", first);  // ← last use of first
v.push(4);              // ✅ first's borrow already ended
```

---

# Section 4 — Structs, Enums & Traits

---

## `.` vs `::` — Method vs Associated Function

```rust
let r = Rectangle::new(5.0, 3.0);  // :: calls an associated function
let a = r.area();                   // .  calls a method on an instance
```

**`::`** — called on the **type itself**, not an instance. No `self` parameter.
Used for: constructors (`Vec::new()`), constants, type-level operations.

**`.`** — called on a **value** (an instance of the type). Has `self` parameter.
Used for: reading data, mutating data, consuming the value.

```rust
impl Rectangle {
    // Associated function — called with ::
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    // Method — called with .
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

---

## `&self` vs `&mut self` vs `self`

These are the three **method receivers** — they decide what the method can do with the struct and what happens to the caller's value after.

```rust
impl MyStruct {
    fn read(&self) { }           // borrows — caller keeps the value, method reads
    fn modify(&mut self) { }     // mutably borrows — caller keeps, method can change
    fn consume(self) { }         // moves — caller loses the value after this call
}
```

In practice:
```rust
let r = Rectangle::new(5.0, 3.0);

r.area();        // &self    — r still valid after
r.scale(2.0);    // &mut self — r still valid but changed
r.into_string(); // self      — r is gone after this call (consumed)
```

**When to use each:**
- `&self` — reading data, computing something from the struct (most common)
- `&mut self` — changing the struct's state
- `self` — consuming/transforming the struct into something else (builder pattern, conversion)

---

## Derive Macros — Auto-generating Trait Implementations

`#[derive]` is a compile-time code generator. It reads your struct definition and writes the trait implementation for you.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: f64, y: f64 }
```

This generates three full trait implementations automatically:

| Derive | What you get | How to use |
|---|---|---|
| `Debug` | Print the struct | `println!("{:?}", p)` or `println!("{:#?}", p)` |
| `Clone` | Deep copy | `let p2 = p1.clone()` |
| `PartialEq` | Equality comparison | `p1 == p2`, `p1 != p2` |
| `Eq` | Stronger equality (also derive PartialEq) | Required for HashMap keys |
| `Hash` | Use as HashMap key | Needed alongside `Eq` |
| `Default` | Zero/empty value | `Point::default()` → `{ x: 0.0, y: 0.0 }` |
| `Copy` | Silent copy on assignment | All fields must also be `Copy` |

Without `#[derive(Debug)]`, `println!("{:?}", p)` is a compile error — `Point` doesn't know how to describe itself.

```rust
struct Point { x: f64, y: f64 }
println!("{:?}", Point { x: 1.0, y: 2.0 }); // ❌ `Point` doesn't implement `Debug`

#[derive(Debug)]
struct Point { x: f64, y: f64 }
println!("{:?}", Point { x: 1.0, y: 2.0 }); // ✅ prints: Point { x: 1.0, y: 2.0 }
```

**Always `#[derive(Debug)]` your structs** during development. It costs nothing and makes debugging infinitely easier.

---

## Enums — Types with Fixed Variants

An enum defines a type that can be **exactly one of a set of named variants**.

```rust
enum Season { Spring, Summer, Autumn, Winter }

let s = Season::Autumn;

let msg = match s {
    Season::Spring => "warm",
    Season::Summer => "hot",
    Season::Autumn => "cool",   // ← matches here
    Season::Winter => "cold",
};
// msg = "cool"
```

Enums can also **carry data** inside each variant:

```rust
enum Shape {
    Circle(f64),                      // holds a radius
    Rectangle { width: f64, height: f64 }, // holds named fields
    Point,                            // holds nothing
}

let s = Shape::Circle(5.0);
match s {
    Shape::Circle(r) => println!("circle with radius {}", r),
    Shape::Rectangle { width, height } => println!("{}x{}", width, height),
    Shape::Point => println!("a point"),
}
```

This is how `Option` and `Result` work — they're just enums:
```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

---

## Nominal Typing — Name is Identity

```rust
struct A { x: i32 }
struct B { x: i32 }  // identical structure to A
```

`A` and `B` are **completely different types** even though they look identical. Rust uses the **name** to identify a type, not its shape.

```rust
fn takes_a(a: A) { println!("{}", a.x); }

let b = B { x: 5 };
takes_a(b);  // ❌ compile error: expected `A`, found `B`
```

### Why this is useful — the newtype pattern

You can create distinct types from the same underlying data to prevent accidental mixing:

```rust
struct Meters(f64);
struct Seconds(f64);

fn speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0
}

let d = Meters(100.0);
let t = Seconds(9.58);
speed(d, t);        // ✅

speed(t, d);        // ❌ compile error: wrong order caught at compile time!
```

Same data, different names → different types → compiler catches mistakes.

---

# Section 5 — Error Handling

---

## Option\<T\> — A Value That Might Not Exist

`Option<T>` replaces `null`. It has two variants:

```rust
Some(value)  // the value exists
None         // the value doesn't exist
```

```rust
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

match safe_divide(10.0, 2.0) {
    Some(result) => println!("Result: {}", result),
    None         => println!("Cannot divide by zero"),
}
```

`Option` forces you to handle both cases. You can never accidentally use a `None` as if it were a value — the compiler won't let you.

---

## Result\<T, E\> — An Operation That Might Fail

`Result<T, E>` is like `Option` but failure has a **reason**:

```rust
Ok(value)   // success — here's the value
Err(error)  // failure — here's why
```

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.trim().parse::<i32>().map_err(|e| e.to_string())
}

match parse_number("42") {
    Ok(n)  => println!("Got: {}", n),
    Err(e) => println!("Failed: {}", e),
}
```

### Option vs Result — when to use which

```
Option → absence is normal and expected (finding an item in a list, optional config)
Result → failure is exceptional and needs a reason (parsing, I/O, network)
```

---

## `.unwrap()`, `?`, and `.unwrap_or()` — The Three Ways to Handle Failure

This is where you dropped two marks. These three behave completely differently.

### `.unwrap()`

Extracts the value if `Ok`/`Some`. **Panics (crashes)** the program if `Err`/`None`.

```rust
let x: Option<i32> = Some(5);
let val = x.unwrap();      // val = 5 ✅

let y: Option<i32> = None;
let val = y.unwrap();      // 💥 PANIC — program crashes immediately
                           // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'
```

When to use: only in tests, prototypes, or when you are **certain** the value exists (and crashing is the right response if it somehow doesn't).

### `?` operator

Extracts the value if `Ok`/`Some`. **Returns the error to the caller** if `Err`/`None`. Does NOT panic.

```rust
fn parse_and_double(s: &str) -> Result<i32, String> {
    let n = s.parse::<i32>().map_err(|e| e.to_string())?;
    //                                                   ↑
    // If Err: immediately returns Err from THIS function
    // If Ok: n gets the value, execution continues
    Ok(n * 2)
}

// The caller receives the Err — no crash
match parse_and_double("abc") {
    Ok(n)  => println!("{}", n),
    Err(e) => println!("error: {}", e),  // ← lands here
}
```

`?` can only be used in functions that return `Result` or `Option`. Using it in `fn main()` (which returns `()` by default) is a compile error:

```rust
fn main() {
    let n = "42".parse::<i32>()?;  // ❌ ? used in function that returns ()
}

// Fix: change main's return type
fn main() -> Result<(), String> {
    let n = "42".parse::<i32>().map_err(|e| e.to_string())?;  // ✅
    Ok(())
}
```

### `.unwrap_or(default)`

Returns the value if `Ok`/`Some`. Returns the **default you provide** if `Err`/`None`. No panic, no early return.

```rust
let x: Option<i32> = None;
let val = x.unwrap_or(0);       // val = 0 — no panic, execution continues

let r: Result<i32, String> = Err("oops".to_string());
let val = r.unwrap_or(-1);      // val = -1 — error silently replaced
```

### Side-by-side

```
.unwrap()              → value  OR  💥 PANIC
?                      → value  OR  ↩ RETURN Err to caller
.unwrap_or(x)          → value  OR  🔄 use default x
.unwrap_or_else(|| x)  → value  OR  🔄 run closure for default
.expect("message")     → value  OR  💥 PANIC with your message
```

### Decision guide

```
Is None/Err impossible here? → .unwrap() or .expect("why it can't happen")
Do I want to pass failure up the call stack? → ?
Do I want a fallback value? → .unwrap_or(default)
Do I want to handle it in this function? → match or if let
```

---

# Section 6 — Iterators & Closures

---

## Lazy Evaluation — Iterators Do Nothing Until Triggered

An iterator chain is a **description of computation**, not the computation itself.
Nothing runs until you call a consuming method at the end.

```rust
let v = vec![1, 2, 3, 4, 5];

// This builds a description — nothing runs yet
let chain = v.iter()
             .filter(|&&x| x % 2 == 0)
             .map(|&x| x * 10);

// THIS triggers execution — the chain runs now
let result: Vec<i32> = chain.collect();
// result = [20, 40]
```

Why lazy? It allows the compiler to **fuse operations** together. `.filter().map()` doesn't create an intermediate `Vec` — it processes one element at a time, all the way through the pipeline. Zero intermediate allocations.

---

## Adapters vs Consumers

Every iterator method is one of two kinds:

### Adapters — transform lazily, return a new iterator

```rust
.map(|x| x * 2)          // transform each element
.filter(|&x| x > 0)      // keep only matching elements
.take(5)                  // take first 5 elements
.skip(2)                  // skip first 2 elements
.enumerate()              // add index: (0, val), (1, val), ...
.zip(other)               // pair with another iterator
.flat_map(|x| ...)        // map + flatten nested iterators
.chain(other)             // append another iterator
```

Adapters do **nothing by themselves**. They just describe what to do.

### Consumers — trigger execution, produce a final value

```rust
.collect::<Vec<_>>()      // gather into a collection
.sum()                    // add everything up
.product()                // multiply everything together
.count()                  // count the elements
.max()                    // find the largest
.min()                    // find the smallest
.any(|x| condition)       // true if any element matches
.all(|x| condition)       // true if all elements match
.find(|x| condition)      // first matching element (Option)
.fold(init, |acc, x| ...) // reduce to a single value
.for_each(|x| ...)        // run a side effect on each element
```

Consumers trigger the entire chain. Once consumed, the iterator is gone.

### Reading an iterator chain

```rust
let result: Vec<i32> = numbers.iter()   // start: yields &i32
    .filter(|&&x| x % 2 == 0)           // adapter: keep even numbers
    .map(|&x| x * 10)                   // adapter: multiply by 10
    .collect();                          // consumer: gather into Vec → RUNS NOW
```

Read it left to right: "take numbers, keep the even ones, multiply by 10, put them in a Vec."

---

## Closures — Inline Functions That Capture Their Environment

A closure is a function you define inline, right where you need it. Syntax: `|params| expression`.

```rust
let double = |x| x * 2;      // closure: takes x, returns x * 2
println!("{}", double(5));    // 10
```

### Closures can capture variables from outside

This is what makes closures different from regular functions:

```rust
let factor = 4;
let multiply = |x| x * factor;  // captures 'factor' from the outer scope
println!("{}", multiply(7));     // 28
println!("{}", multiply(3));     // 12
// factor is still available here too
```

Regular functions can't do this — they can only use their parameters and global constants.

### Closures used with iterators

```rust
let numbers = vec![1, 2, 3, 4, 5];

// closure passed to map:
let doubled: Vec<i32> = numbers.iter()
    .map(|&x| x * 2)     // |&x| x * 2 is a closure
    .collect();
// [2, 4, 6, 8, 10]

// closure can capture from outside:
let threshold = 3;
let big: Vec<i32> = numbers.iter()
    .filter(|&&x| x > threshold)  // captures 'threshold'
    .copied()
    .collect();
// [4, 5]
```

### The `&&x` and `&x` pattern explained

When you call `.iter()` on a `Vec<i32>`, it yields `&i32` (references to each element).
When that `&i32` goes into `.filter()`, it gets wrapped in another reference — so inside the closure you get `&&i32`.

```rust
numbers.iter()                   // yields: &i32
       .filter(|&&x| x > 0)     // filter wraps in &: gets &&i32, pattern matches down to i32
       .map(|&x| x * 2)         // map wraps in &: gets &i32, pattern matches down to i32
```

The `&&x` pattern in filter destructures through both layers of references to get the plain `i32` value `x`. It's one of Rust's rough edges — don't stress it, just recognise the pattern.

---

## Writing Iterator Chains — The Full Pattern

The `sum_of_even_squares` function from Q34, built step by step:

```rust
fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    numbers.iter()               // 1. create iterator — yields &i32
           .filter(|&&x| x % 2 == 0)  // 2. keep evens: 2, 4
           .map(|&x| x * x)     // 3. square them: 4, 16
           .sum()                // 4. add them up: 20
}

sum_of_even_squares(&[1, 2, 3, 4, 5]);  // → 20
```

The rule: **chain adapters → end with a consumer**. The consumer is what makes it run.

---

# Master Quick Reference

## Memory

```
Stack  → fast, automatic, fixed size (i32, bool, arrays, tuples of stack types)
Heap   → flexible, managed by ownership (String, Vec, HashMap)
&T     → a reference: just a memory address — lives on the stack, points to heap (or stack)
*x     → dereference: follow the reference, operate on the real value
```

## Copy vs Move

```
Copy types:  i32 f64 bool char tuples/arrays of Copy types
             Assignment duplicates silently. Original still valid.

Move types:  String Vec HashMap (anything owning heap memory)
             Assignment transfers ownership. Original is gone.

Explicit copy of a Move type: .clone()
```

## Borrowing Rules

```
&T      → immutable borrow   — unlimited at the same time
&mut T  → mutable borrow     — exactly one, no others active simultaneously
Never both &T and &mut T at the same time
Borrow ends at last use (NLL), not at closing }
```

## Functions — How to Pass Values

```rust
fn f(x: T)        // move in — caller loses x
fn f(x: &T)       // borrow — caller keeps x, read-only
fn f(x: &mut T)   // mut borrow — caller keeps x, function can modify
fn f(x: &str)     // string slice borrow — most flexible for strings
```

## Error Handling

```
.unwrap()       → value or PANIC
?               → value or RETURN Err to caller (function must return Result/Option)
.unwrap_or(x)   → value or USE default x
.expect("msg")  → value or PANIC with your message
```

## Iterator Pattern

```
collection.iter()          ← start
  .filter(|&&x| cond)      ← adapter (lazy)
  .map(|&x| transform)     ← adapter (lazy)
  .sum() / .collect()      ← consumer (triggers execution)
```

---

# Exercises — Write These Without Notes

If you can do all five cleanly, you've got it.

**Exercise 1 — Borrowing:**
Write a function that takes a `&Vec<String>` and returns the longest string as `Option<&str>`.
Don't clone anything.

**Exercise 2 — Mutable Borrowing:**
Write a function that takes a `&mut Vec<i32>` and removes all negative numbers in place.
Hint: use `.retain()`.

**Exercise 3 — Ownership + Structs:**
Write a `BankAccount` struct with a `balance: f64` field.
Implement `deposit(&mut self, amount: f64)`, `withdraw(&mut self, amount: f64) -> Result<f64, String>` (error if insufficient funds), and `balance(&self) -> f64`.

**Exercise 4 — Error Handling:**
Write a function `parse_pair(s: &str) -> Result<(i32, i32), String>` that:
- Splits the input on `","`
- Parses both halves as `i32`
- Returns them as a tuple
- Uses `?` for error propagation

**Exercise 5 — Iterators:**
Write a function that takes a `&[String]` and returns the count of strings that:
- Have more than 3 characters AND
- Start with an uppercase letter

Use a single iterator chain. No loops.
