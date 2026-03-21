# Rust Fundamentals — Deep Dive Reference

A structured walkthrough of Rust from first principles, built through 39 hands-on problems. Each section links to the relevant Rust documentation and includes the exact code patterns from this repo.

---

## Table of Contents

1. [Why Rust](#1-why-rust)
2. [Toolchain & Project Structure](#2-toolchain--project-structure)
3. [Variables, Mutability & Shadowing](#3-variables-mutability--shadowing)
4. [Primitive Types](#4-primitive-types)
5. [Compound Types — Tuples, Arrays, Slices](#5-compound-types--tuples-arrays-slices)
6. [Functions & Expressions](#6-functions--expressions)
7. [Control Flow](#7-control-flow)
8. [Strings — `String` vs `&str`](#8-strings--string-vs-str)
9. [Ownership](#9-ownership)
10. [Borrowing & References](#10-borrowing--references)
11. [Structs & impl Blocks](#11-structs--impl-blocks)
12. [Enums & Pattern Matching](#12-enums--pattern-matching)
13. [Option\<T\>](#13-optiont)
14. [Collections — Vec, HashMap, HashSet](#14-collections--vec-hashmap-hashset)
15. [Error Handling — Result\<T, E\> & the ? Operator](#15-error-handling--resultt-e--the--operator)
16. [Generics & Trait Bounds](#16-generics--trait-bounds)
17. [Traits](#17-traits)
18. [Lifetimes](#18-lifetimes)
19. [Closures & Iterators](#19-closures--iterators)
20. [Problem Index](#20-problem-index)

---

## 1. Why Rust

Rust is a **compiled systems language** with three non-negotiable design goals:

- **Performance** — compiles to native machine code, no runtime overhead
- **Memory safety** — no garbage collector; memory bugs are compile errors, not runtime crashes
- **Correctness** — the type system and borrow checker catch entire classes of bugs before execution

### Comparison to other languages

| Property | JavaScript/Python | C/C++ | Rust |
|---|---|---|---|
| Memory management | Garbage collector | Manual (`malloc`/`free`) | Ownership system |
| Error detection | Mostly at runtime | Mostly at runtime | Mostly at compile time |
| Concurrency safety | Runtime / library | Programmer responsibility | Compile-time borrow rules |
| Performance | Interpreted / JIT | Native | Native |
| String indexing | Silent `undefined` | UB / segfault | Compile/runtime error |

### The compiler is the tool

> "If it compiles, it probably works."

Rust moves bugs from runtime to compile time. You write more explicit code upfront, but you get far fewer surprises in production. This is the correct mental model for learning the language.

**Docs:** [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/) · [Why Rust?](https://doc.rust-lang.org/book/ch00-00-introduction.html)

---

## 2. Toolchain & Project Structure

### Tools

```
rustup   — toolchain version manager (like nvm)
cargo    — build system + package manager + test runner (like npm + make)
```

**Docs:** [rustup](https://rustup.rs/) · [The Cargo Book](https://doc.rust-lang.org/cargo/)

### Initialise a project

```bash
cargo init          # in existing directory
cargo new my_proj   # creates new directory
```

### Standard layout

```
my_proj/
├── Cargo.toml      # package manifest
└── src/
    └── main.rs     # binary entry point
```

### Cargo.toml

```toml
[package]
name = "rust-solana"
version = "0.1.0"
edition = "2024"

[dependencies]

# Multiple binaries — one file per problem:
[[bin]]
name = "variables"
path = "src/variables.rs"
```

### Key commands

```bash
cargo run                  # compile + run src/main.rs
cargo run --bin NAME       # compile + run a specific [[bin]]
cargo build                # compile only → target/debug/
cargo build --release      # optimised build → target/release/
cargo check                # type-check without producing a binary (fast)
cargo test                 # run all tests
```

**Docs:** [Cargo manifest format](https://doc.rust-lang.org/cargo/reference/manifest.html)

---

## 3. Variables, Mutability & Shadowing

**Docs:** [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

### Immutable by default

```rust
let x = 5;
// x = 6;  // compile error: cannot assign twice to immutable variable
```

Every `let` binding is immutable unless explicitly declared otherwise. This is not a limitation — it's a signal to the reader (and the compiler) that the value won't change.

### Mutable bindings

```rust
let mut total = 0;
total += 10;  // allowed
```

`mut` must appear at the binding site AND at the call site when passing a mutable reference:

```rust
let mut data = vec![1, 2, 3];
double_all(&mut data);   // both mut keywords required
```

### Shadowing

```rust
let s = "  42  ";          // &str
let s = s.trim();          // still &str, but different value
let s: i32 = s.parse().unwrap();  // now i32 — type changed
let s = s * s;             // back to i32
```

Key distinction:

| | `mut` | shadowing (`let`) |
|---|---|---|
| Reassigns value | yes | yes |
| Can change type | no | yes |
| Creates new variable | no | yes |
| Old binding reusable | n/a | no |

**Code:** `src/mutability.rs` (P3), `src/shadowing.rs` (P4)

---

## 4. Primitive Types

**Docs:** [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

### Numeric types

| Category | Types | Default inference |
|---|---|---|
| Signed integer | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `i32` |
| Unsigned integer | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | — |
| Float | `f32`, `f64` | `f64` |

`isize`/`usize` match the platform's pointer width (64-bit on modern hardware). Use `usize` for indexing and lengths.

**Overflow behaviour:**
- Literal out of range → **compile error**
- Arithmetic overflow at runtime → **panic in debug mode**, wraps in release mode
- Use checked arithmetic when you need to handle overflow gracefully: `.checked_add()`, `.wrapping_add()`, `.saturating_add()`

```rust
let x: i8 = 200;       // compile error
let mut n: i8 = 100;
n = n + 100;            // runtime panic (debug) or wrap (release)
```

### Boolean

```rust
let flag: bool = true;
if flag && other { ... }   // && = AND, || = OR, ! = NOT
```

No truthy/falsy coercion — conditions must be `bool`.

### Character

```rust
let c: char = 'é';
```

`char` is a **Unicode scalar value** — always 4 bytes, regardless of the character. This is different from C (`u8`) and JavaScript (UTF-16 code unit). A `String` is not an array of `char`s.

**Code:** `src/primitives.rs` (P5)

---

## 5. Compound Types — Tuples, Arrays, Slices

**Docs:** [The Tuple Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type) · [The Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) · [Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)

### Tuples

Fixed-length, heterogeneous. Primary use case: returning multiple values from a function.

```rust
fn swap(a: i32, b: i32) -> (i32, i32) { (b, a) }

let (x, y) = swap(1, 2);   // destructuring
let first = pair.0;         // index access
```

### Arrays

Fixed-length, homogeneous. Type syntax: `[T; N]`. Lives on the stack.

```rust
let data: [i32; 3] = [10, 20, 30];
let first = data[0];   // panics if out of bounds
```

### Slices

A **borrowed view** into a contiguous sequence. Type: `&[T]`. Works with both arrays and `Vec<T>`.

```rust
fn first_last(nums: &[i32]) -> (i32, i32) {
    (nums[0], nums[nums.len() - 1])
}

// call with array:
first_last(&[1, 2, 3]);
// call with Vec:
first_last(&vec![1, 2, 3]);
```

Always prefer `&[T]` over `&Vec<T>` in function signatures — slices are more general.

**Code:** `src/tuples.rs` (P6), `src/arrays.rs` (P7)

---

## 6. Functions & Expressions

**Docs:** [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

### Syntax

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // last expression, no semicolon = return value
}
```

- Every parameter requires a type annotation
- Return type follows `->`
- The **last expression without a semicolon** is the implicit return value
- `return` exists for early returns but is not idiomatic at the end of a function

### Expressions vs statements

This is fundamental to Rust. Almost everything is an expression (produces a value):

```rust
let y = {
    let x = 3;
    x + 1       // expression — evaluates to 4
};              // y = 4
```

Adding `;` turns an expression into a statement (produces `()` — the unit type):

```rust
let y = { x + 1; };   // y = (), not 4 — common mistake
```

**Code:** `src/functions.rs` (P8)

---

## 7. Control Flow

**Docs:** [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) · [match](https://doc.rust-lang.org/book/ch06-02-match.html)

### if as an expression

```rust
let label = if score > 90 { "great" } else { "ok" };
```

Both branches must return the same type — the compiler enforces this. No ternary operator needed.

### Loops

```rust
loop { break; }             // infinite loop; break can return a value
while condition { }
for i in 0..10 { }          // exclusive end: 0–9
for i in 0..=10 { }         // inclusive end: 0–10
for _ in 0..5 { }           // _ suppresses unused variable warning
for (i, val) in v.iter().enumerate() { }  // position + value
```

`loop` can return a value:
```rust
let result = loop {
    if done { break 42; }
};
```

### match

```rust
match value {
    "penny"  => 1,
    "nickel" => 5,
    "dime" | "ten" => 10,    // | combines patterns
    90..=100 => "A",          // range patterns
    _        => 0,            // wildcard — must cover all remaining cases
}
```

`match` is exhaustive — the compiler rejects non-exhaustive matches. `match` is also an expression — all arms return the same type.

**Code:** `src/if_expr.rs` (P9), `src/loops.rs` (P10), `src/match_expr.rs` (P17), `src/enums.rs` (P19)

---

## 8. Strings — `String` vs `&str`

**Docs:** [What is a String?](https://doc.rust-lang.org/book/ch08-02-strings.html)

### The two types

| Type | Storage | Owned? | Growable? | Common use |
|---|---|---|---|---|
| `String` | Heap | Yes | Yes | Owned string you build or modify |
| `&str` | Anywhere (often binary/stack) | No (borrowed) | No | Read-only view; function parameters |

```rust
let s1: &str = "hello";              // string literal — &'static str
let s2: String = String::from("hi"); // owned, heap-allocated
let s3: &str = &s2;                  // borrow a String as &str (auto-deref)
```

### Always use `&str` in function signatures

```rust
fn process(s: &str) { ... }   // accepts &str AND &String
fn process(s: &String) { ... } // only accepts &String — unnecessarily restrictive
```

### Mutation

```rust
let mut s = String::from("Hell");
s.push('o');           // append char
s.push_str(" world");  // append &str
let combined = s + " suffix";  // moves s, appends &str
```

### No direct indexing

Rust strings are UTF-8. A `char` is 1–4 bytes. `s[0]` is ambiguous (byte? char? grapheme?), so Rust disallows it.

```rust
let s = String::from("café");
s.len()               // 5 bytes
s.chars().count()     // 4 Unicode scalar values ← correct for "characters"
s.chars().nth(3)      // Option<char> — returns Some('é')
&s[0..3]              // &str slice by bytes — panics if mid-char boundary
```

### Formatting

```rust
let msg = format!("{}: {} cents", name, price);  // returns String
println!("{}", msg);   // {} = Display trait
println!("{:?}", val); // {:?} = Debug trait (structs, Vec, Option, etc.)
```

**Code:** `src/strings.rs` (P21), `src/slices.rs` (P15), `src/ownership.rs` (P12)

---

## 9. Ownership

**Docs:** [Understanding Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

Ownership is Rust's memory management model. No garbage collector, no manual `free`. The rules:

1. Every value has exactly **one owner**
2. When the owner goes out of scope, the value is **automatically dropped** (memory freed)
3. Assigning a heap value to another variable **moves** it — the original is no longer valid

### Move semantics

```rust
let s1 = String::from("hello");
let s2 = s1;           // s1 is MOVED into s2
// println!("{}", s1); // compile error: s1 was moved
```

### Copy types

Stack-only types (`i32`, `f64`, `bool`, `char`, fixed-size arrays, tuples of `Copy` types) implement the `Copy` trait. Assignment copies rather than moves:

```rust
let x = 5;
let y = x;   // x is copied, not moved
println!("{}", x);  // fine
```

### Ownership through functions

```rust
fn takes_ownership(s: String) { ... }  // s is dropped at end of function
fn makes_copy(n: i32) { ... }          // n is copied, caller still has it

let s = String::from("hi");
takes_ownership(s);
// println!("{}", s);  // compile error: s was moved
```

To keep using a value after passing it to a function: either return it back, or use borrowing (see §10).

**Code:** `src/ownership.rs` (P12)

---

## 10. Borrowing & References

**Docs:** [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

Borrowing lets you access a value without taking ownership.

### Immutable reference

```rust
fn length(s: &str) -> usize { s.len() }  // borrows, does not move

let s = String::from("hello");
println!("{}", length(&s));  // s still valid here
```

### Mutable reference

```rust
fn double_all(nums: &mut Vec<i32>) {
    for n in nums.iter_mut() { *n *= 2; }
}

let mut data = vec![1, 2, 3];  // binding must be `mut`
double_all(&mut data);          // borrow site must be `&mut`
```

`*n` dereferences the reference to modify the value behind it.

### The borrow rules (compile-time enforcement)

At any given point in a program:
- You may have **any number of immutable references** (`&T`), OR
- You may have **exactly one mutable reference** (`&mut T`)
- But never both at the same time

This eliminates data races — at compile time, zero runtime cost.

```rust
let mut v = vec![1, 2, 3];
let r1 = &v;
let r2 = &v;         // ok — multiple immutable refs
// let r3 = &mut v;  // compile error: v already borrowed immutably
println!("{} {}", r1[0], r2[0]);  // r1 and r2 used here
let r3 = &mut v;     // ok — r1 and r2 are no longer used after this point
```

**Code:** `src/borrowing.rs` (P13), `src/mut_borrowing.rs` (P14)

---

## 11. Structs & impl Blocks

**Docs:** [Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) · [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

### Defining a struct

```rust
struct Rectangle {
    width: i32,
    height: i32,
}

let r = Rectangle { width: 4, height: 5 };
println!("{}", r.width);
```

All fields must be provided at construction. Structs own their fields — the same ownership/copy rules apply.

### Methods (`&self`)

```rust
impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

r.area();       // 20
r.is_square();  // false
```

- `&self` — immutable borrow; reads but does not modify
- `&mut self` — mutable borrow; can modify fields
- `self` — takes ownership; consumes the instance

### Associated functions (constructors)

```rust
impl Rectangle {
    fn square(size: i32) -> Self {
        Self { width: size, height: size }
    }
}

let sq = Rectangle::square(6);  // called with ::, not dot
```

No `new` keyword in Rust — `::new()` and similar are by convention, not language requirement. `Self` refers to the implementing type.

**Code:** `src/structs.rs` (P16, P16b, P16c)

---

## 12. Enums & Pattern Matching

**Docs:** [Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) · [The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)

### Enums carry data

Unlike C-style enums, Rust enum variants can hold data of different types:

```rust
enum Shape {
    Circle(f64),           // tuple variant
    Rectangle(i32, i32),   // tuple variant
    Point { x: i32, y: i32 },  // struct variant
}
```

### Destructuring in match

```rust
match shape {
    Shape::Circle(r) => println!("circle r={}", r),
    Shape::Rectangle(w, h) => println!("{}x{}", w, h),
    Shape::Point { x, y } => println!("({}, {})", x, y),
}
```

### Range patterns

```rust
fn get_grade(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89  => "B",
        _        => "F",
    }
}
```

**Code:** `src/match_expr.rs` (P17), `src/enums.rs` (P19)

---

## 13. Option\<T\>

**Docs:** [The Option Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)

Rust has no `null`. Absence is represented by `Option<T>`:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The compiler forces you to handle `None`. You cannot use an `Option<i32>` where an `i32` is expected.

```rust
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}
```

### Consuming Option

```rust
// Explicit handling:
match result {
    Some(v) => println!("{}", v),
    None    => println!("nothing"),
}

// Shorthand methods:
result.unwrap()              // value or panic
result.unwrap_or(0)          // value or default
result.unwrap_or_else(|| 0)  // value or computed default
result.is_some()             // bool
result.is_none()             // bool
result.map(|v| v * 2)        // transform Some, pass through None
```

**Code:** `src/option.rs` (P18)

---

## 14. Collections — Vec, HashMap, HashSet

**Docs:** [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

### Vec\<T\>

Dynamic array. Heap-allocated, growable.

```rust
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];   // macro shorthand

v.push(4);
v.pop();              // Option<i32>
v.len();
v[0];                 // panics if out of bounds
v.get(0)              // Option<&i32> — safe access
```

Pass as `&[T]` (slice) to functions — more general than `&Vec<T>`.

### HashMap\<K, V\>

```rust
use std::collections::HashMap;

let mut map: HashMap<String, i32> = HashMap::new();
map.insert("key".to_string(), 42);
map.get("key")                      // Option<&i32>
map.contains_key("key")             // bool

// Insert-or-update pattern:
*map.entry("key".to_string()).or_insert(0) += 1;
```

### HashSet\<T\>

```rust
use std::collections::HashSet;

let unique: HashSet<&str> = text.split_whitespace().collect();
unique.len()             // distinct element count
unique.contains("word")
```

**Code:** `src/vectors.rs` (P20), `src/hashmaps.rs` (P22)

---

## 15. Error Handling — Result\<T, E\> & the ? Operator

**Docs:** [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

Rust has no exceptions. Recoverable errors use `Result<T, E>`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Returning errors

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| "invalid number".to_string())
}
```

`.map_err(f)` transforms the `Err` variant while leaving `Ok` unchanged.

### The ? operator

```rust
fn add_parsed(a: &str, b: &str) -> Result<i32, String> {
    let a = a.parse::<i32>().map_err(|_| "parse error".to_string())?;
    let b = b.parse::<i32>().map_err(|_| "parse error".to_string())?;
    Ok(a + b)
}
```

`?` desugars to: if `Ok(v)` → bind `v` and continue; if `Err(e)` → return `Err(e)` immediately. The function return type must be `Result` (or `Option`). This is the standard error propagation pattern in real Rust code.

### Result vs Option

| Type | Use when |
|---|---|
| `Option<T>` | A value may or may not be present |
| `Result<T, E>` | An operation may succeed or fail, and failure has a reason |

### Consuming Result

```rust
result.unwrap()            // value or panic
result.unwrap_or(default)  // value or fallback
result.is_ok()             // bool
result.is_err()            // bool
result.map(|v| v * 2)      // transform Ok, pass through Err
result.map_err(|e| ...)    // transform Err, pass through Ok
match result { Ok(v) => ..., Err(e) => ... }
```

**Code:** `src/result.rs` (P23), `src/question_mark.rs` (P24), `src/csv_parse.rs` (P25), `src/calculator.rs` (P39)

---

## 16. Generics & Trait Bounds

**Docs:** [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)

### Generic functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list { if item > max { max = item; } }
    max
}
```

`T: PartialOrd` is a **trait bound** — constrains `T` to types that support `>`. The compiler verifies the constraint at every call site.

### Multiple bounds

```rust
fn print_and_return<T: Display + Clone>(val: T) -> T {
    println!("{}", val);
    val.clone()
}
```

### Where clauses (for readability with complex bounds)

```rust
fn process<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Debug + PartialOrd,
{ ... }
```

**Code:** `src/generics.rs` (P26)

---

## 17. Traits

**Docs:** [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)

Traits are Rust's equivalent of interfaces. They define shared behaviour across types.

### Defining and implementing

```rust
trait Describable {
    fn describe(&self) -> String;  // signature only
}

impl Describable for Item {
    fn describe(&self) -> String {
        format!("{}: {} cents", self.name, self.price)
    }
}
```

### Using traits as parameters

```rust
fn print_desc(item: &impl Describable) { println!("{}", item.describe()); }
// equivalent:
fn print_desc<T: Describable>(item: &T) { println!("{}", item.describe()); }
```

### The #[derive] attribute

Many standard traits can be auto-implemented:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
struct Point { x: i32, y: i32 }
```

| Trait | Enables |
|---|---|
| `Debug` | `{:?}` formatting |
| `Clone` | `.clone()` |
| `Copy` | implicit copy on assignment (stack types only) |
| `PartialEq` / `Eq` | `==` and `!=` |
| `Hash` | use as `HashMap` key |
| `Default` | `Type::default()` zero value |
| `PartialOrd` / `Ord` | `<`, `>`, sorting |

### Associated types

```rust
trait Summarize {
    type Output;
    fn summarize(&self) -> Self::Output;
}

impl Summarize for Numbers {
    type Output = i32;
    fn summarize(&self) -> i32 { self.data.iter().sum() }
}
```

Associated types (`type Output`) vs generic parameters (`<T>`): associated types mean each type can only implement the trait once, and the output type is inferred at call sites without annotation. The `Iterator` trait uses `type Item` for exactly this reason.

**Code:** `src/traits.rs` (P27), `src/derive.rs` (P28), `src/assoc_types.rs` (P29)

---

## 18. Lifetimes

**Docs:** [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

Lifetimes tell the borrow checker how long references are valid. They prevent dangling references at compile time.

### `'static`

```rust
fn classify(n: i32) -> &'static str {
    if n > 0 { "positive" } else { "negative" }
}
```

`'static` means valid for the entire program. String literals are always `'static` — they're baked into the binary.

### Lifetime parameters

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
```

`'a` is a lifetime parameter. It does not allocate anything — it is purely a constraint telling the compiler: "the returned reference is valid for at least as long as both `x` and `y` are valid."

Without `'a`, the compiler cannot know whether the returned reference borrows from `x` or `y`, so it rejects the function.

### Lifetime elision rules

The compiler infers lifetimes automatically in common cases. Manual annotation is needed when:

1. **Rule 1:** Each `&` parameter gets its own lifetime
2. **Rule 2:** If there is exactly one input lifetime, it applies to all outputs
3. **Rule 3:** If one input is `&self`, the `self` lifetime applies to outputs

When there are multiple reference inputs and a reference output, rules 2 and 3 may not apply — annotation required.

```rust
// Elision works — single input:
fn first_word(s: &str) -> &str { ... }

// Must annotate — two reference inputs, output borrows from one:
fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str { ... }
```

**Code:** `src/lifetimes.rs` (P30), `src/lifetime_annotations.rs` (P31), `src/elision.rs` (P32)

---

## 19. Closures & Iterators

**Docs:** [Closures](https://doc.rust-lang.org/book/ch13-01-closures.html) · [Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

### Closures

Anonymous functions that can capture their environment:

```rust
let factor = 3;
let triple = |x| x * factor;   // captures factor by reference
```

Closure syntax: `|params| expression` or `|params| { block }`. Types are inferred.

### Iterator pipelines

Iterators in Rust are **lazy** — nothing executes until a consuming method is called.

```rust
nums.iter()              // creates iterator yielding &i32
    .map(|x| x * x)      // wraps in MapIterator — no work done yet
    .filter(|x| x > &10) // wraps in FilterIterator — no work done yet
    .sum()               // consuming adapter — drives the whole pipeline
```

No intermediate allocations between `.map()` and `.filter()`. The entire chain compiles down to a single loop.

### Key iterator methods

| Method | Description |
|---|---|
| `.map(f)` | Transform each element |
| `.filter(f)` | Keep elements where `f` returns `true` |
| `.fold(init, f)` | Accumulate to a single value |
| `.sum()` | Sum all elements (requires `Sum` trait) |
| `.collect::<C>()` | Consume into collection `C` (Vec, HashSet, etc.) |
| `.find(f)` | First element satisfying predicate → `Option<T>` |
| `.any(f)` / `.all(f)` | Boolean tests across elements |
| `.enumerate()` | Pairs each element with its index |
| `.take(n)` | Stop after `n` elements |
| `.zip(other)` | Pair elements from two iterators |
| `.flat_map(f)` | Map then flatten |
| `.copied()` | `&T → T` for `Copy` types |
| `.cloned()` | `&T → T` for `Clone` types |

### Reference layers in filter

`.iter()` yields `&T`. `.filter()` gives its closure `&&T` (an extra `&`). Two ways to handle:

```rust
// Option 1: double-destructure
nums.iter().filter(|&&x| x > 0)

// Option 2: deref in predicate
nums.iter().filter(|x| **x > 0)

// Option 3 (cleanest): move .copied() before .filter()
nums.iter().copied().filter(|x| *x > 0)
```

### Implementing Iterator

Only `next` is required. All other methods come for free:

```rust
impl Iterator for Countdown {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.n > 0 { let v = self.n; self.n -= 1; Some(v) }
        else { None }
    }
}
// Now Countdown works with for loops, .collect(), .map(), .sum(), etc.
```

**Code:** `src/closures.rs` (P33), `src/iterators.rs` (P34), `src/filter.rs` (P35), `src/custom_iter.rs` (P36)

---

## 20. Problem Index

All problems are in `src/`. Run any with `cargo run --bin <name>`.

| # | File | Concept |
|---|---|---|
| P1 | `main.rs` | Hello World, `println!` macro |
| P2 | `variables.rs` | `let`, type inference, `{}` formatter |
| P3 | `mutability.rs` | `let mut`, accumulator, inclusive range `..=` |
| P4 | `shadowing.rs` | Shadowing, type change, `.trim()`, `.parse()` |
| P5 | `primitives.rs` | `i64`, primitive type table |
| P6 | `tuples.rs` | Tuple return, destructuring |
| P7 | `arrays.rs` | `&[T]` slices, `.len()` |
| P8 | `functions.rs` | Function signatures, `if` as expression |
| P9 | `if_expr.rs` | `if` expression, unit type `()` |
| P10 | `loops.rs` | `for`, `while`, `loop`, `u64` overflow |
| P11 | `fizzbuzz.rs` | Conditionals, `String::from`, `.to_string()` |
| P12 | `ownership.rs` | Move semantics, `String + &str` |
| P13 | `borrowing.rs` | `&str`, `.chars().count()`, Unicode |
| P14 | `mut_borrowing.rs` | `&mut`, `*deref`, `.iter_mut()`, borrow rules |
| P15 | `slices.rs` | `.split_whitespace()`, `.next()`, `.unwrap_or()` |
| P16 | `structs.rs` | `struct`, `impl`, `&self`, `Self`, associated fn |
| P17 | `match_expr.rs` | `match`, exhaustiveness, wildcard `_`, `\|` patterns |
| P18 | `option.rs` | `Option<T>`, `Some`, `None` |
| P19 | `enums.rs` | Range patterns, `&'static str` |
| P20 | `vectors.rs` | `Vec<T>`, `.push()`, `&[T]` |
| P21 | `strings.rs` | `.chars()`, `.filter()`, case normalisation |
| P22 | `hashmaps.rs` | `HashSet`, `.collect()`, turbofish |
| P23 | `result.rs` | `Result<T,E>`, `.map_err()` |
| P24 | `question_mark.rs` | `?` operator, error propagation |
| P25 | `csv_parse.rs` | Combined error handling, `format!` |
| P26 | `generics.rs` | `<T: PartialOrd>`, trait bounds |
| P27 | `traits.rs` | `trait`, `impl Trait for Type` |
| P28 | `derive.rs` | `#[derive]`, `PartialEq`, `Clone`, `Debug` |
| P29 | `assoc_types.rs` | Associated types, `type Item` |
| P30 | `lifetimes.rs` | `'static`, string literals |
| P31 | `lifetime_annotations.rs` | `'a` parameter, dangling reference prevention |
| P32 | `elision.rs` | Lifetime elision rules |
| P33 | `closures.rs` | `\|x\|` syntax, capture, `.map().collect()` |
| P34 | `iterators.rs` | `.map().sum()`, lazy pipelines, `.fold()` |
| P35 | `filter.rs` | `.filter()`, `&&T`, `.copied()` |
| P36 | `custom_iter.rs` | `impl Iterator`, `fn next`, associated type |
| P37 | `capstone.rs` | HashMap, `.entry().or_insert()`, `.find()` |
| P38 | `diagonal.rs` | Nested `.collect()`, 2D indexing |
| P39 | `calculator.rs` | Full error handling pipeline |

---

## Key Mental Models

### Ownership in one sentence
Every value is owned by exactly one variable. When that variable goes out of scope, the value is freed. Assignment of heap data *moves* the owner; it does not copy.

### Borrowing in one sentence
A reference lets you read (`&T`) or modify (`&mut T`) a value without taking ownership. The borrow checker ensures you never have a mutable reference active at the same time as any other reference.

### Option vs Result
- `Option<T>` — the value might not exist
- `Result<T, E>` — the operation might fail, and failure has a reason

### Iterators in one sentence
Iterators are lazy values that produce elements one at a time; chaining `.map().filter()` builds a description of a computation, not the computation itself — a terminal method (`.sum()`, `.collect()`, `for`) triggers execution.

### Lifetimes in one sentence
Lifetimes are compile-time annotations that tell the borrow checker how long a reference is valid; they do not change runtime behaviour, only the compiler's ability to reject invalid programs.

---

## Further Reading

- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/) — primary reference
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — code-first learning
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — unsafe Rust and memory model
- [std library docs](https://doc.rust-lang.org/std/) — full standard library reference
- [Crates.io](https://crates.io) — package registry
- [Rust Playground](https://play.rust-lang.org/) — run Rust in the browser
- [Rustlings](https://github.com/rust-lang/rustlings) — interactive exercises
