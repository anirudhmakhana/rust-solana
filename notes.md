# Rust Notes

---

## Part 1: Intro and Hello World

### Why Rust

Rust is a compiled systems language designed for performance, memory safety, and strong compile-time checks.

- Strong type safety — variables cannot silently change type
- Compiles to a native binary
- Fast at runtime (compiled ahead of time)
- Low-level system resource access
- First-class concurrency with threads
- Memory safety without a garbage collector

### Important mindset

**"If it compiles, it probably works."**
Rust pushes many bugs into compile-time errors. You do more work before runtime, fewer surprises after deployment.

Rust exposes low-level concerns directly. More control, more correctness, more explicitness.

### Toolchain

- `rustup` — toolchain manager (like `nvm`)
- `cargo` — build tool + package manager (like `npm`, but also builds, runs, tests)

### Project structure

```bash
cargo init
```

Creates:
- `Cargo.toml` — package manifest (like `package.json`)
- `src/main.rs` — executable entry point

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Running programs

```bash
cargo run              # build + execute
cargo build            # build only → target/debug/
cargo build --release  # optimized → target/release/
cargo run --bin NAME   # run a specific binary
```

### Hello World

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn` declares a function
- `main` is the program entry point
- `println!` is a macro (note the `!`) — expands at compile time
- Semicolons end statements

### Multiple binaries in one project

Add to `Cargo.toml`:

```toml
[[bin]]
name = "my_program"
path = "src/my_program.rs"
```

Then run with `cargo run --bin my_program`.

---

## Part 2: Variables, Types, Strings, Conditionals, Loops, Functions

### Variables with `let`

```rust
let x = 5;        // type inferred (i32)
let y: i32 = 5;   // type explicit
```

Variables are **immutable by default**. To reassign, use `mut`:

```rust
let mut x = 5;
x = 6; // ok
```

### Number types

Rust exposes multiple numeric types — memory layout matters in systems programming.

| Category | Types |
|---|---|
| Signed integers | `i8`, `i16`, `i32`, `i64`, `i128` |
| Unsigned integers | `u8`, `u16`, `u32`, `u64`, `u128` |
| Floats | `f32`, `f64` |

Default inference: integer literals → `i32`, float literals → `f64`.

`i8` range: -128 to 127. `u8` range: 0 to 255.

**Compile-time vs runtime overflow:**
- Out-of-range literal → compile error
- Overflow from runtime arithmetic → runtime panic (in dev mode)

### Booleans

```rust
let is_active = true;
if is_active && some_flag { ... }  // && = AND, || = OR
```

Condition must be `bool` — no truthy/falsy coercion like JavaScript.

### Strings

Two main string types:

| Type | Description |
|---|---|
| `String` | Owned, heap-allocated, growable |
| `&str` | Borrowed string slice (string literals are `&str`) |

```rust
let s = String::from("Hello");  // owned String
let s = "Hello";                // &str
```

Growing a `String` (requires `mut`):

```rust
let mut s = String::from("Hell");
s.push('o');          // add one char
s.push_str(" world"); // add a string slice
```

**No direct indexing** — use `.chars()` instead:

```rust
let ch = s.chars().nth(0); // returns Option<char>, not char
```

Handling `Option<char>`:

```rust
match ch {
    Some(c) => println!("{}", c),
    None => println!("not found"),
}
// or shortcut (panics if None):
let c = s.chars().nth(0).unwrap();
```

### Conditionals

```rust
if n % 2 == 0 {
    println!("even");
} else {
    println!("odd");
}
```

No parentheses needed around the condition.

### Loops

```rust
for i in 0..10 { ... }   // 0 to 9 (end is exclusive)
for _ in 0..10 { ... }   // _ ignores the variable
```

Iterating over a string:

```rust
for ch in s.chars() { ... }
for (i, ch) in s.chars().enumerate() { ... }  // position + char
```

### Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon = return expression
}
```

Parameter types and return type are required. Last expression without `;` is the return value.

---

## Problems

### P1 — Hello World (`src/main.rs`)

Print `Hello, Rust!`.

```rust
fn main() {
    println!("Hello, Rust!");
}
```

Key idea: `println!` is a macro.

---

### P2 — Variables (`src/variables.rs`)

Bind `x = 42` and `name = "Rust"`, print both.

```rust
fn main() {
    let x = 42;
    let name = "Rust";
    println!("x = {}", x);
    println!("name = {}", name);
}
```

Key ideas:
- `let` binds a value to a name
- Type is inferred: `x` → `i32`, `name` → `&str` (string literal)
- `{}` is the format placeholder in `println!`
- No `mut` needed — values are never changed

---

### P3 — Mutability (`src/mutability.rs`)

Compute the sum of 1 to n using a mutable accumulator.

```rust
fn sum_to(n: i32) -> i32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}
```

Key ideas:
- `let mut` is required to reassign or modify a variable
- `1..=n` is an **inclusive** range (includes `n`); `1..n` would exclude `n`
- `total` on the last line with no `;` is the implicit return value
- Without `mut`, `total += i` would be a compile error

---

### P4 — Shadowing (`src/shadowing.rs`)

Trim, parse, and square a string value using shadowing at each step.

```rust
fn transform(s: &str) -> i32 {
    let s = s.trim();           // &str → &str (whitespace removed)
    let s: i32 = s.parse().unwrap();  // &str → i32
    let s = s * s;              // i32 → i32 (squared)
    s
}
```

Key ideas:
- Each `let s = ...` creates a **new binding** that shadows the previous one
- Shadowing can change the type — `s` goes from `&str` to `i32`; `mut` cannot do this
- `"  7  ".trim()` returns a `&str` with leading/trailing whitespace removed
- `.parse()` returns `Result<i32, _>` — `.unwrap()` extracts the value or panics on failure
- Shadowing vs `mut`: `mut` keeps the same variable and type; shadowing replaces the binding entirely

---

### P5 — Primitives (`src/primitives.rs`)

Multiply two `i64` values.

```rust
fn multiply(a: i64, b: i64) -> i64 {
    a * b
}
```

Key ideas:
- `i64` holds values up to ~9.2 × 10¹⁸ — use it when `i32` might overflow
- `char` in Rust is a full Unicode scalar (4 bytes), not a single ASCII byte like in C
- Primitive type summary:

| Category | Types | Default inference |
|---|---|---|
| Signed int | `i8`, `i16`, `i32`, `i64`, `i128` | `i32` |
| Unsigned int | `u8`, `u16`, `u32`, `u64`, `u128` | — |
| Float | `f32`, `f64` | `f64` |
| Boolean | `bool` | — |
| Character | `char` | — |

---

### P6 — Tuples (`src/tuples.rs`)

Return two values in swapped order using a tuple.

```rust
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}
```

Key ideas:
- Tuples group values of **different types** into one compound value: `(i32, &str, bool)`
- A function can return a tuple to hand back multiple values
- **Destructuring** unpacks a tuple into named bindings: `let (x, y) = swap(1, 2);`
- **Index access** uses `.0`, `.1`, etc.: `pair.0`
- Tuples are fixed-length — you cannot add or remove elements after creation

---

### P7 — Arrays & Slices (`src/arrays.rs`)

Return the first and last elements of a slice.

```rust
fn first_last(nums: &[i32]) -> (i32, i32) {
    (nums[0], nums[nums.len() - 1])
}
```

Key ideas:
- `[i32; 3]` is an **array** — fixed size, known at compile time: `let data = [10, 20, 30];`
- `&[i32]` is a **slice** — a borrowed view into any contiguous sequence (array or Vec)
- Passing `&data` converts the array into a slice — slices are the idiomatic way to accept sequences in functions
- Index access with `nums[0]` panics at runtime if out of bounds (no silent undefined like JS)
- `.len()` returns the number of elements; last index is always `len() - 1`
- Slices do not own their data — they borrow it (hence the `&`)

---

### P8 — Functions (`src/functions.rs`)

Return the largest of three `i32` values.

```rust
fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= c {
        b
    } else {
        c
    }
}
```

Key ideas:
- Every parameter requires a type; return type comes after `->`
- `if/else if/else` is an **expression** in Rust — each branch returns a value, no `return` needed
- The last expression in a branch (no `;`) becomes the value of that branch
- All branches must return the same type — the compiler enforces this
- `return` exists for early exits but is not idiomatic at the end of a function

---

### P9 — if as an expression (`src/if_expr.rs`)

Return the absolute value of `n` using `if` as an expression.

```rust
fn abs_value(n: i32) -> i32 {
    if n < 0 { -n } else { n }
}
```

Key ideas:
- `if` is an **expression** — it produces a value that can be returned directly or assigned
- Both branches must have the same type; mismatched types are a compile error
- No semicolon on the branch values — adding `;` would make the branch return `()` (unit), breaking the type
- Equivalent to a ternary (`n < 0 ? -n : n`) in other languages, but more readable
- Assigning from `if`: `let label = if score > 90 { "great" } else { "ok" };`

---

### P10 — Loops (`src/loops.rs`)

Compute factorial using a `for` loop.

```rust
fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}
```

Key ideas:
- `loop` — infinite loop, exit with `break`; can return a value: `let x = loop { break 42; };`
- `while cond { }` — runs while condition is true
- `for i in 1..=n { }` — inclusive range; `1..n` excludes `n`
- Starting at `2` skips the no-op `*= 1` and handles `n = 0` and `n = 1` correctly (result stays 1)
- `u64` chosen because factorials grow fast — `20!` already exceeds `i32::MAX`

---

### P11 — FizzBuzz (`src/fizzbuzz.rs`)

Return "Fizz", "Buzz", "FizzBuzz", or the number as a `String`.

```rust
fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}
```

Key ideas:
- Check `% 15` first — if you check `% 3` first, `n = 15` would return "Fizz" and never reach "FizzBuzz"
- `String::from("...")` creates an owned `String` from a string literal
- `n.to_string()` converts any type that implements `Display` into an owned `String`
- The function returns `String` (owned), not `&str` (borrowed) — necessary because the number branch builds a new string at runtime
- The whole `if/else` chain is an expression; all branches return `String`

---

### P12 — Ownership (`src/ownership.rs`)

Take ownership of two strings and return them concatenated with a space.

```rust
fn join_strings(a: String, b: String) -> String {
    a + " " + &b
}
```

Key ideas:
- **Ownership rules**: every value has one owner; when the owner goes out of scope, the value is dropped
- Passing a `String` to a function **moves** it — the caller can no longer use it after the call
- The `+` operator on `String` consumes the left-hand side (`a` is moved into the result) and appends a `&str`
- `&b` coerces `String` → `&str` for the second `+`; `b` is borrowed, not moved
- After `join_strings(a, b)`, both `a` and `b` are gone — their ownership transferred into the function
- This is why `String` concatenation looks asymmetric: `String + &str`, not `String + String`

---

### P13 — Borrowing (`src/borrowing.rs`)

Count Unicode characters in a string by borrowing it.

```rust
fn count_chars(s: &str) -> usize {
    s.chars().count()
}
```

Key ideas:
- `&` creates a **reference** — the function borrows the value without taking ownership
- The caller keeps ownership; the value is not dropped after the call
- `&str` accepts both string literals and `&String` (auto-deref) — prefer `&str` in function signatures
- `.len()` returns **bytes**, not characters — `"café".len()` = 5, `"café".chars().count()` = 4
- `.chars()` iterates over Unicode scalar values; `.count()` consumes the iterator and returns the total
- `usize` is the standard type for counts and lengths — it matches the platform's pointer size

---

### P14 — Mutable Borrowing (`src/mut_borrowing.rs`)

Multiply every element in a vector by 2, modifying it in place.

```rust
fn double_all(nums: &mut Vec<i32>) {
    for n in nums.iter_mut() {
        *n *= 2;
    }
}
```

Key ideas:
- `&mut T` is a **mutable reference** — lets you modify the value without taking ownership
- Call site must also be explicit: `double_all(&mut data)` — mutability is opt-in everywhere
- `*n` **dereferences** the reference to reach the actual value; without `*`, you'd be working on the reference itself
- `.iter_mut()` yields `&mut i32` references to each element, allowing in-place mutation
- Rust's borrow rule: **one mutable reference OR any number of immutable references — never both at once**
- This rule is enforced at compile time, preventing data races without a runtime cost
- `vec![1, 2, 3]` is the macro shorthand for creating a `Vec<i32>`
- `{:?}` in `println!` uses the `Debug` format — prints `Vec` as `[2, 4, 6, 8, 10]`

---

### P15 — Slices (`src/slices.rs`)

Extract the first whitespace-separated word from a string.

```rust
fn first_word(s: &str) -> String {
    s.split_whitespace()
        .next()
        .unwrap_or(s)
        .to_string()
}
```

Key ideas:
- `&str[0..5]` is a **string slice** — a borrowed view into a portion of string data
- `.split_whitespace()` returns an iterator of `&str` words, automatically handling multiple/leading spaces
- `.next()` pulls the first item from an iterator, returning `Option<&str>`
- `.unwrap_or(s)` provides a fallback if the `Option` is `None` (empty string case) — safer than `.unwrap()`
- `.to_string()` converts `&str` → `String` (owned) to match the return type
- The method chain avoids manual index arithmetic and handles edge cases (leading spaces, empty input) cleanly

---

### P16 — Structs (`src/structs.rs`)

Define a `Rectangle` struct and compute its area.

```rust
struct Rectangle {
    width: i32,
    height: i32,
}

fn rect_area(w: i32, h: i32) -> i32 {
    let rect = Rectangle { width: w, height: h };
    rect.width * rect.height
}
```

Key ideas:
- `struct` defines a custom type with named fields — each field has an explicit type
- Instantiate with `TypeName { field: value, ... }` — all fields must be provided
- Access fields with `.` dot notation: `rect.width`
- Structs own their data — the same ownership rules apply (fields with `String` get moved, fields with `i32` get copied)
- Next step: `impl Rectangle { fn area(&self) -> i32 { ... } }` — attaching methods to a struct

#### P16b — impl blocks (added to `src/structs.rs`)

```rust
impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```

Key ideas:
- `impl TypeName { }` attaches methods to a struct
- `&self` is an immutable borrow of the instance — the method reads but doesn't modify
- `&mut self` would be used if the method needed to mutate fields
- Access fields through `self.field` inside methods
- Call with dot notation: `r.area()`, `r.is_square()`
- Multiple methods live in one `impl` block (or you can have multiple `impl` blocks for the same type)

#### P16c — Associated functions (added to `src/structs.rs`)

```rust
fn square(size: i32) -> Self {
    Self { width: size, height: size }
}
```

Key ideas:
- **Associated function**: no `self` parameter — belongs to the type, not an instance
- Called with `::` syntax: `Rectangle::square(6)`, not `r.square(6)`
- `Self` refers to the type being implemented (`Rectangle`) — cleaner than repeating the type name
- Commonly used as constructors/factory methods (Rust has no `new` keyword, this is the convention)
- `String::from(...)`, `Vec::new()` are both associated functions — you've been using them all along

---

### P17 — match expression (`src/match_expr.rs`)

Map coin names to their cent values using `match`.

```rust
fn coin_value(coin: &str) -> i32 {
    match coin {
        "penny" => 1,
        "nickel" => 5,
        "dime" => 10,
        "quarter" => 25,
        _ => 0,
    }
}
```

Key ideas:
- `match` compares a value against **patterns** and runs the first arm that matches
- `_` is the wildcard — catches any value not matched above; required when not all cases are covered
- The compiler enforces **exhaustiveness** — every possible value must be handled or the code won't compile
- `|` combines multiple patterns in one arm: `"Sat" | "Sun" => "weekend"`
- `match` is an **expression** — it returns the value of the matched arm (no `;` on arm values)
- All arms must return the same type

---

### P18 — Option (`src/option.rs`)

Return `Some(result)` or `None` to represent a division that may fail.

```rust
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
```

Key ideas:
- `Option<T>` is an enum with two variants: `Some(T)` (value exists) and `None` (no value)
- There is no `null` in Rust — absence is always represented explicitly in the type
- The caller is forced to handle both cases; you cannot accidentally use a `None` as a value
- Common ways to consume an `Option`:
  - `match` — handle `Some(v)` and `None` explicitly
  - `.unwrap()` — extract value or panic if `None`
  - `.unwrap_or(default)` — extract value or use a fallback
  - `.is_some()` / `.is_none()` — check without extracting
- `{:?}` prints `Some(5)` and `None` directly via the `Debug` trait

---

### P19 — Enums & range patterns (`src/enums.rs`)

Map a numeric score to a letter grade using `match` with range patterns.

```rust
fn get_grade(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        _        => "F",
    }
}
```

Key ideas:
- `90..=100` is an **inclusive range pattern** inside `match` — matches any value in that range
- `..=0` means "up to and including 0" (no lower bound needed)
- `&'static str` — the `'static` lifetime means the string lives for the entire program; string literals always have this lifetime
- Enums in Rust can carry data inside variants: `enum Shape { Circle(f64), Rect(i32, i32) }` — much more powerful than C-style enums
- `match` on enums with data uses destructuring: `Shape::Circle(r) => ...`

---

### P20 — Vectors (`src/vectors.rs`)

Sum all elements in a slice.

```rust
fn sum_vec(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}
```

Key ideas:
- `Vec<T>` is a heap-allocated growable array; `vec![1, 2, 3]` is the shorthand macro
- Functions accept `&[T]` (slice) instead of `&Vec<T>` — slices work with both `Vec` and plain arrays
- `for n in nums` on a slice yields `&i32` references; Rust auto-derefs in `total += n`
- Common `Vec` operations: `.push(val)`, `.pop()` → `Option<T>`, `.len()`, `.is_empty()`
- Iterator shorthand for the same result: `nums.iter().sum()`

---

### P21 — Strings (`src/strings.rs`)

Count vowels in a string regardless of case.

```rust
fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiou".contains(c.to_ascii_lowercase())).count()
}
```

Key ideas:
- `.chars()` gives an iterator over Unicode `char` values
- `.filter(|c| ...)` keeps only elements where the closure returns `true`; `|c|` is closure syntax (like an arrow function)
- `c.to_ascii_lowercase()` normalises case before checking — handles both `'A'` and `'a'`
- `"aeiou".contains(char)` checks membership — `&str` implements `contains` for both `char` and `&str`
- The whole chain — `.chars().filter(...).count()` — is a **iterator pipeline**; no intermediate collection is created
- `String` vs `&str`: accept `&str` in function signatures; it works for both string literals and `&String` (auto-deref)

---

### P22 — HashMaps & HashSets (`src/hashmaps.rs`)

Count distinct words in a string using a `HashSet`.

```rust
use std::collections::HashSet;

fn unique_word_count(s: &str) -> usize {
    s.split_whitespace().collect::<HashSet<_>>().len()
}
```

Key ideas:
- `HashSet<T>` stores unique values — inserting a duplicate is a no-op
- `HashMap<K, V>` stores key-value pairs; `.get(key)` returns `Option<&V>`
- Both require `use std::collections::{HashMap, HashSet}` — not in the prelude by default
- `.collect::<HashSet<_>>()` — `.collect()` consumes an iterator into a collection; the turbofish `::<HashSet<_>>` tells the compiler which collection type to build (`_` lets it infer the element type)
- Common `HashMap` operations:
  - `.insert(k, v)` — add or overwrite
  - `.get(k)` → `Option<&V>`
  - `.entry(k).or_insert(default)` — insert only if key is absent (useful for counting)

---

### P23 — Result (`src/result.rs`)

Parse a string to `i32`, returning `Ok` or a custom `Err`.

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| "invalid number".to_string())
}
```

Key ideas:
- `Result<T, E>` has two variants: `Ok(T)` on success, `Err(E)` on failure — no exceptions
- `.parse::<i32>()` returns `Result<i32, ParseIntError>`; the turbofish specifies the target type
- `.map_err(|e| ...)` transforms the error variant while leaving `Ok` untouched — used to convert error types
- `|_|` ignores the original error value (we don't need it, just want our custom message)
- Common ways to consume a `Result`:
  - `match` — handle both arms explicitly
  - `.unwrap()` — extract `Ok` or panic on `Err`
  - `.unwrap_or(default)` — extract `Ok` or use a fallback
  - `?` operator — propagate `Err` up to the calling function (covered later)
- `Result` vs `Option`: `Option` = value may be absent; `Result` = operation may fail with an error

---

### P24 — The `?` operator (`src/question_mark.rs`)

Parse two strings and return their sum, propagating errors early.

```rust
fn add_parsed(a: &str, b: &str) -> Result<i32, String> {
    let a = a.parse::<i32>().map_err(|_| "parse error".to_string())?;
    let b = b.parse::<i32>().map_err(|_| "parse error".to_string())?;
    Ok(a + b)
}
```

Key ideas:
- `?` at the end of a `Result` expression: if `Ok(v)` → unwraps to `v` and continues; if `Err(e)` → returns `Err(e)` immediately from the function
- The function must return `Result` (or `Option`) for `?` to work
- `?` replaces a verbose `match` — without it, each parse would need its own `match` block
- The error types must match (or implement `From` conversion) — `.map_err()` handles the conversion here
- `Ok(a + b)` at the end — the happy path must be explicitly wrapped in `Ok`
- This is the standard Rust error propagation pattern; real code uses `?` extensively

---

### P25 — Combining error handling (`src/csv_parse.rs`)

Parse a comma-separated string of numbers and sum them, with descriptive errors.

```rust
fn parse_csv_sum(s: &str) -> Result<i32, String> {
    if s.is_empty() {
        return Err("empty input".to_string());
    }
    let mut sum = 0;
    for token in s.split(',') {
        let n = token
            .trim()
            .parse::<i32>()
            .map_err(|_| format!("invalid number: {}", token.trim()))?;
        sum += n;
    }
    Ok(sum)
}
```

Key ideas:
- Early `return Err(...)` for the empty case — checked before any iteration
- `.split(',')` splits on a char delimiter, yielding `&str` tokens
- `.trim()` handles any whitespace around commas (e.g. `"1, 2, 3"`)
- `format!("invalid number: {}", token)` builds a dynamic error string — like `println!` but returns a `String`
- `?` short-circuits the loop on the first bad token — the rest are never processed
- This pattern (validate → iterate → accumulate → return `Ok`) is the standard shape for fallible processing

---

### P26 — Generics (`src/generics.rs`)

Find the largest element in a slice.

```rust
fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &n in &list[1..] {
        if n > max {
            max = n;
        }
    }
    max
}
```

Key ideas:
- `for &n in &list[1..]` — the `&n` pattern **destructures** the `&i32` reference, binding `n` as a plain `i32` copy
- `list[1..]` is a slice starting from index 1 — skips the first element already stored in `max`
- The generic version would be `fn largest<T: PartialOrd>(list: &[T]) -> T` — `T: PartialOrd` is the trait bound that allows `>`
- Trait bounds (`T: Trait`) constrain generics to types that implement specific behaviour — the compiler verifies this
- `i32` is `Copy`, so `max = n` copies the value; for non-`Copy` types (like `String`) you'd need references

---

### P27 — Traits (`src/traits.rs`)

Define a trait and implement it for a struct.

```rust
trait Describable {
    fn describe(&self) -> String;
}

struct Item { name: String, price: i32 }

impl Describable for Item {
    fn describe(&self) -> String {
        format!("{}: {} cents", self.name, self.price)
    }
}
```

Key ideas:
- `trait Trait { fn method(&self) -> ReturnType; }` — declares the interface; no implementation body
- `impl Trait for Type { ... }` — provides the concrete implementation for that type
- Any number of types can implement the same trait; any type can implement many traits
- Trait bounds in functions: `fn print_desc(item: &impl Describable)` or `fn print_desc<T: Describable>(item: &T)`
- Built-in traits you'll use constantly: `Display` (for `{}`), `Debug` (for `{:?}`), `Clone`, `Copy`, `Iterator`
- Traits are how Rust achieves polymorphism without inheritance

---

### P28 — Derive (`src/derive.rs`)

Auto-implement traits with `#[derive]` and use them in functions.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

fn are_equal(a: &Point, b: &Point) -> bool { a == b }

fn distance_sq(a: &Point, b: &Point) -> i32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}
```

Key ideas:
- `#[derive(...)]` is a **proc macro attribute** that generates trait implementations automatically
- `PartialEq` enables `==` and `!=` — without it, comparing two `Point`s is a compile error
- `Clone` enables `.clone()` — explicit deep copy; `Copy` (for stack-only types like `i32`) makes copies implicit
- `Debug` enables `{:?}` formatting — essential for printing structs during development
- Squared distance avoids `f64` and `sqrt` — useful when you only need to compare distances, not the actual value
- Common derivable traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Default`, `PartialOrd`, `Ord`

---

### P29 — Associated types (`src/assoc_types.rs`)

Define a trait with an associated type, implemented differently for two structs.

```rust
trait Summarize {
    type Output;
    fn summarize(&self) -> Self::Output;
}

impl Summarize for Numbers {
    type Output = i32;
    fn summarize(&self) -> i32 { self.data.iter().sum() }
}

impl Summarize for Sentence {
    type Output = String;
    fn summarize(&self) -> String { self.words.join(" ") }
}
```

Key ideas:
- `type Output;` inside a trait declares an **associated type** — a placeholder resolved per implementation
- `Self::Output` refers to whatever type the implementor sets `Output` to
- Each type can only implement the trait once (vs a generic `<T>` which could be implemented for `T=i32`, `T=String`, etc.)
- No need to write `Summarize<i32>` at call sites — the output type is inferred from the concrete type
- `.iter().sum()` works because `i32` implements the `Sum` trait — the compiler infers the sum type
- `.join(" ")` on `Vec<String>` produces a single `String` with the separator inserted between elements
- The `Iterator` trait itself uses this pattern: `type Item` is the associated type

---

### P30 — Lifetimes & `'static` (`src/lifetimes.rs`)

Return a `&'static str` from a function using string literals.

```rust
fn classify(n: i32) -> &'static str {
    if n > 0 { "positive" } else if n < 0 { "negative" } else { "zero" }
}
```

Key ideas:
- `'static` is a **lifetime** — it means the reference is valid for the entire duration of the program
- String literals (`"hello"`) are baked into the compiled binary, so they always have `'static` lifetime
- Lifetimes are how Rust tracks how long references are valid — they prevent dangling references at compile time
- `'static` is the longest possible lifetime; most lifetimes are shorter and tied to a scope
- Non-`'static` lifetime syntax: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` — `'a` says "the output lives as long as both inputs"
- You've been using `'static` since P19 (`get_grade`) — this problem makes the concept explicit

---

### P31 — Lifetime annotations (`src/lifetime_annotations.rs`)

Return whichever of two string slices is longer, with explicit lifetime annotation.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
```

Key ideas:
- `'a` is a **lifetime parameter** — it names a scope that the compiler will infer at each call site
- `-> &'a str` says "the returned reference borrows from the same scope as `x` and `y`"
- The compiler uses this to ensure the returned reference doesn't outlive the data it points to
- Without `'a`, the compiler can't know whether the return borrows from `x` or `y`, so it rejects the function
- The annotation doesn't change how the function runs — it's purely information for the borrow checker
- Lifetime elision: for single-input functions Rust infers the lifetime automatically; multi-input functions that return a reference usually need explicit annotations

---

### P32 — Lifetime elision (`src/elision.rs`)

Strip a prefix from a string, returning a slice tied to the original string's lifetime.

```rust
fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    if s.starts_with(prefix) {
        &s[prefix.len()..]
    } else {
        s
    }
}
```

Key ideas:
- `prefix` has no lifetime annotation — the output never borrows from it, so the compiler doesn't need to track it
- `'a` only annotates `s` and the return — explicitly saying "output borrows from `s`, not `prefix`"
- `&s[prefix.len()..]` — slices `s` from the prefix length to the end, returning a `&str` into `s`'s memory
- **Elision rules** (when the compiler infers lifetimes automatically):
  1. Each `&` input gets its own lifetime
  2. If there's exactly one input lifetime, it applies to all outputs
  3. If one input is `&self`, its lifetime applies to all outputs
- This function needs explicit `'a` because rule 2 doesn't apply (two reference inputs) and there's no `self`

---

### P33 — Closures (`src/closures.rs`)

Double every element in a slice using `.map()` and a closure.

```rust
fn double_all(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|x| x * 2).collect()
}
```

Key ideas:
- `|x| x * 2` is a **closure** — an anonymous function; `x` is inferred as `&i32` from the iterator
- Closures capture variables from the surrounding scope: `let factor = 3; nums.iter().map(|x| x * factor)`
- `.iter()` yields `&i32` references; the `*` dereference is implicit in arithmetic
- `.map(f)` transforms each element lazily — nothing runs until consumed
- `.collect()` drives the iterator and gathers results into a `Vec<i32>` (type inferred from return type)
- Common iterator methods: `.map()`, `.filter()`, `.fold()`, `.sum()`, `.any()`, `.all()`, `.find()`, `.enumerate()`
- Iterators are lazy — chaining `.map().filter()` builds a pipeline with no intermediate allocations

---

### P34 — Iterators (`src/iterators.rs`)

Square each element and sum the results with a single iterator pipeline.

```rust
fn sum_of_squares(nums: &[i32]) -> i32 {
    nums.iter().map(|x| x * x).sum()
}
```

Key ideas:
- `.map(|x| x * x)` transforms each `&i32` — arithmetic auto-derefs the reference
- `.sum()` is a **consuming adapter** — it drives the iterator to completion and returns the total
- `.fold()` is the general version: `nums.iter().fold(0, |acc, x| acc + x * x)` — same result, explicit accumulator
- The whole pipeline is **zero-allocation** — no intermediate `Vec` is created between `.map()` and `.sum()`
- Iterator chaining mental model: each method wraps the previous in a new lazy iterator; only the final consumer (`.sum()`, `.collect()`, `for`) actually runs the computation

---

### P35 — Filter (`src/filter.rs`)

Keep only even numbers using `.filter()`.

```rust
fn evens_only(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x % 2 == 0).copied().collect()
}
```

Key ideas:
- `.filter()` receives `&&i32` — a reference to a reference — because `.iter()` yields `&i32` and `.filter()` adds another layer of `&`
- `|&&x|` double-destructures: the outer `&` from `.filter()`, the inner `&` from `.iter()`, binding `x` as plain `i32`
- `.copied()` converts `&i32` elements to `i32` (works for `Copy` types); `.cloned()` is the equivalent for `Clone` types like `String`
- Alternatively: `.filter(|x| *x % 2 == 0).copied()` — single `&` destructure with explicit dereference
- Or use `.iter().copied().filter(|x| x % 2 == 0)` — call `.copied()` first to simplify the closure to `|x: i32|`

---

### P36 — Custom Iterator (`src/custom_iter.rs`)

Implement `Iterator` for a `Countdown` struct that yields n down to 1.

```rust
impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let current = self.n;
            self.n -= 1;
            Some(current)
        } else {
            None
        }
    }
}
```

Key ideas:
- `Iterator` only requires one method: `fn next(&mut self) -> Option<Self::Item>`
- Return `Some(value)` to yield the next item; return `None` to signal the iterator is exhausted
- `&mut self` is needed because iterators advance state — `n` must be decremented each call
- Once you implement `Iterator`, you get `.map()`, `.filter()`, `.collect()`, `for` loops, and all other iterator methods **for free**
- Save the current value before decrementing — otherwise you'd return the decremented value
- The `for n in Countdown::new(5)` loop desugars to calling `.next()` repeatedly until `None`

---

### P37 — Capstone: Text Analysis (`src/capstone.rs`)

Find the most frequently occurring word, returning the first in case of ties.

```rust
fn most_frequent(s: &str) -> String {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in s.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    let max_count = *counts.values().max().unwrap();

    s.split_whitespace()
        .map(|w| w.to_lowercase())
        .find(|w| counts[w.as_str()] == max_count)
        .unwrap()
}
```

Key ideas:
- `.entry(k).or_insert(0)` — inserts 0 if key absent, returns `&mut usize`; `*` dereferences to increment
- `.values().max()` — finds the highest count across all entries; returns `Option<&usize>`, so `*` to deref
- `.find()` returns the **first** element satisfying the predicate — correct for the tie-break rule
- `.max_by_key()` would return the **last** maximum — wrong for "first appearing" tie-break
- `counts[w.as_str()]` — `HashMap` supports `&str` indexing even when keyed by `String` via the `Borrow` trait
- This problem combines: `HashMap`, iterator pipelines, closures, `String`/`&str` conversion, and `Option` handling

---

### P38 — Capstone: 2D Data Processing (`src/diagonal.rs`)

Parse a matrix from a string and sum both diagonals, subtracting the center once if odd-sized.

```rust
fn diagonal_sum(input: &str) -> i32 {
    let matrix: Vec<Vec<i32>> = input
        .split(';')
        .map(|row| row.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let n = matrix.len();
    let mut sum = 0;
    for i in 0..n {
        sum += matrix[i][i];          // primary diagonal
        sum += matrix[i][n - 1 - i];  // secondary diagonal
    }
    if n % 2 == 1 {
        sum -= matrix[n / 2][n / 2];  // center counted twice
    }
    sum
}
```

Key ideas:
- Nested `.collect()` — outer collects rows into `Vec<Vec<i32>>`, inner collects values into `Vec<i32>`; type annotation on `matrix` tells the compiler what to build
- Primary diagonal index: `[i][i]`; secondary diagonal index: `[i][n-1-i]` — both walk from opposite corners
- `n % 2 == 1` detects odd dimension; center is at `[n/2][n/2]` (integer division)
- Even matrices have no shared center element, so no subtraction needed
- This problem combines: string splitting, nested iterator maps, 2D indexing, and arithmetic reasoning about diagonals

---

### P39 — Capstone: Calculator (`src/calculator.rs`)

Parse and evaluate a simple arithmetic expression with full error handling.

```rust
fn evaluate(expr: &str) -> Result<i32, String> {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("invalid expression".to_string());
    }
    let a = parts[0].parse::<i32>().map_err(|_| "invalid expression".to_string())?;
    let op = parts[1];
    let b = parts[2].parse::<i32>().map_err(|_| "invalid expression".to_string())?;
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => if b == 0 { Err("division by zero".to_string()) } else { Ok(a / b) },
        _   => Err("unknown operator".to_string()),
    }
}
```

Key ideas:
- Validate structure first (`parts.len() != 3`) before attempting to parse — fail fast on malformed input
- `?` propagates parse errors early; all three error types produce different messages
- `match op { "+" => ..., _ => Err(...) }` — `match` on `&str` patterns is clean and exhaustive
- Division by zero is checked inside the `/` arm — it's a runtime condition, not a parse error
- This problem ties together: `split_whitespace`, `collect`, `.parse()`, `.map_err()`, `?`, and `match` — the full error-handling toolkit

---

# Week 2 — Advanced Rust: Smart Pointers & Concurrency

---

## Foundation: Stack vs Heap (refresh before every problem this week)

Every value in Rust lives in one of two places. This matters constantly in week 2.

```
STACK                          HEAP
─────────────────────          ──────────────────────────────
Fast. Fixed size.              Slower. Dynamic size.
Cleaned up automatically       You control when it's freed
when scope ends.               (Rust does it via Drop).

let x: i32 = 5;               let s = String::from("hi");
┌───────────┐                  STACK          HEAP
│  x = 5   │                  ┌─────────┐    ┌───────────┐
└───────────┘                  │ ptr  ───────▶│  "hi"     │
                               │ len = 2 │    └───────────┘
                               │ cap = 2 │
                               └─────────┘

The String struct (ptr + len + cap) lives on the stack.
The actual text data lives on the heap.
```

**Rule of thumb:**
- Known size at compile time → stack (`i32`, `bool`, `[i32; 3]`, tuples of those)
- Unknown/dynamic size → heap (`String`, `Vec<T>`, `Box<T>`, `Rc<T>`, `Arc<T>`)

---

## Part 3: Smart Pointers

A **smart pointer** is a struct that wraps a pointer but adds extra behaviour — like automatic cleanup, reference counting, or runtime borrow checking. They all implement `Deref` (so you can use them like a regular reference) and `Drop` (so memory is freed automatically).

| Type | Owners | Thread-safe? | Mutation | Use when |
|---|---|---|---|---|
| `Box<T>` | 1 | Yes | Normal rules | Heap alloc, recursive types, trait objects |
| `Rc<T>` | Many | No | Immutable only | Multiple owners in single-threaded code |
| `RefCell<T>` | 1 | No | Interior (runtime) | Need to mutate through a shared reference |
| `Rc<RefCell<T>>` | Many | No | Interior (runtime) | Multiple owners + mutation, single-thread |
| `Arc<T>` | Many | Yes | Immutable only | Multiple owners across threads |
| `Mutex<T>` | 1 lock | Yes | Exclusive (locked) | Mutable shared state across threads |
| `Arc<Mutex<T>>` | Many | Yes | Exclusive (locked) | The standard shared mutable state pattern |

---

### W2-P1 — Box\<T\> (`src/box_type.rs`)

**[Docs: Box\<T\>](https://doc.rust-lang.org/book/ch15-01-box.html)**

Heap-allocate a recursive linked list and sum its values.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn list_sum(list: &List) -> i32 {
    match list {
        List::Cons(val, rest) => val + list_sum(rest),
        List::Nil => 0,
    }
}
```

#### Why Box is needed here

Without `Box`, the compiler rejects the enum:

```
// BROKEN — compile error
enum List {
    Cons(i32, List),   // List contains List contains List... infinite size
    Nil,
}
```

The compiler needs to know the byte size of every type at compile time. A `List` that contains a `List` has no fixed size — it's infinite. `Box` breaks the cycle:

```
// FIXED
enum List {
    Cons(i32, Box<List>),   // Box is always pointer-sized: 8 bytes on 64-bit
    Nil,
}
```

#### Memory layout of Cons(3, Cons(2, Cons(1, Nil)))

```
STACK                HEAP
┌──────────────┐
│ list         │
│  tag: Cons   │
│  val: 3      │
│  ptr ────────────▶ ┌──────────────┐
└──────────────┘     │  tag: Cons   │
                     │  val: 2      │
                     │  ptr ────────────▶ ┌──────────────┐
                     └──────────────┘     │  tag: Cons   │
                                          │  val: 1      │
                                          │  ptr ────────────▶ ┌──────┐
                                          └──────────────┘     │  Nil │
                                                               └──────┘
```

Each `Box` is just a pointer (8 bytes on the stack/heap) that owns the next node on the heap. When the outermost `Box` is dropped, it chains-drops everything recursively.

#### Key ideas

- `Box::new(val)` moves `val` to the heap; returns a `Box<T>` (stack pointer to heap data)
- `Box` implements `Deref` → you can use `*b` or just `.field` and Rust auto-derefs
- `Box` implements `Drop` → when `Box` goes out of scope, the heap allocation is freed automatically
- When pattern matching on `&List`, Rust auto-derefs `&Box<List>` → `&List` — no `*` needed
- Three core uses of `Box`:
  1. **Recursive types** (this problem)
  2. **Trait objects**: `Box<dyn Draw>` — store any type implementing `Draw` behind a pointer
  3. **Large data**: move a huge struct to the heap to avoid stack overflow

---

### W2-P2 — Deref trait (`src/deref_trait.rs`)

**[Docs: Deref trait](https://doc.rust-lang.org/book/ch15-02-deref.html)**

Implement `Deref` on a wrapper type so it transparently coerces to its inner type.

```rust
use std::ops::Deref;

struct Wrapper<T>(T);

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn double_len(s: &str) -> usize { s.len() * 2 }

fn main() {
    let w = Wrapper(String::from("hello"));
    println!("{}", double_len(&w)); // 10
}
```

#### What deref coercion looks like in memory

```
You write:        double_len(&w)
                             │
                    &Wrapper<String>
                             │  Rust calls w.deref()
                             ▼
                          &String       ← Wrapper's Deref impl returns &self.0
                             │  Rust calls String's built-in Deref
                             ▼
                           &str         ← what double_len actually receives
```

Rust applies as many deref steps as needed — at compile time, zero runtime cost.

#### Key ideas

- `Deref` has one required method: `fn deref(&self) -> &Self::Target`
- `*x` desugars to `*(x.deref())` — the `*` operator calls `deref()` under the hood
- **Deref coercion** happens automatically when passing `&T` where `&U` is expected, if `T: Deref<Target=U>`
- The coercion chain in this problem: `&Wrapper<String>` → `&String` → `&str` (two hops)
- Built-in coercions you already use: `&String` → `&str`, `&Vec<T>` → `&[T]`, `&Box<T>` → `&T`
- `DerefMut` is the mutable version: `fn deref_mut(&mut self) -> &mut Self::Target`
- Tuple struct field access: `self.0` is the first (and only) field of `Wrapper<T>(T)`

---

### W2-P3 — Rc<T> (`src/rc_type.rs`)

**[Docs: Rc\<T\>](https://doc.rust-lang.org/book/ch15-04-rc.html)**

Multiple owners of the same heap data via reference counting.

```rust
use std::rc::Rc;

fn count_owners(n: usize) -> usize {
    let shared = Rc::new("shared".to_string());
    let _clones: Vec<Rc<String>> = (0..n).map(|_| Rc::clone(&shared)).collect();
    Rc::strong_count(&shared)
}
```

#### Memory layout — what Rc actually looks like

```
Normal Box (one owner):           Rc (multiple owners):

STACK        HEAP                 STACK        HEAP
┌───────┐    ┌──────────┐         ┌────────┐   ┌───────────────────┐
│  box ─────▶│  "data"  │         │  rc1 ──────▶│ strong_count: 3   │
└───────┘    └──────────┘         └────────┘   │ weak_count:   0   │
                                  ┌────────┐   │ data: "shared"    │
                                  │  rc2 ──────▶│                   │
                                  └────────┘   └───────────────────┘
                                  ┌────────┐          ▲
                                  │  rc3 ─────────────┘
                                  └────────┘

All three Rc pointers point to the SAME heap allocation.
Rc::clone only increments strong_count — no data is copied.
When count reaches 0, the data is freed.
```

#### Key ideas

- `Rc<T>` = reference-counted pointer — multiple owners, single-threaded only
- `Rc::clone(&rc)` increments the count; does NOT copy the inner data (unlike `.clone()` on a String)
- `Rc::strong_count(&rc)` returns the current owner count
- When an `Rc` is dropped, count decrements; at 0, heap data is freed
- `Rc<T>` gives **immutable** access only — you cannot mutate through a plain `Rc`
- For mutation with multiple owners → `Rc<RefCell<T>>` (next problem)
- **NOT thread-safe** — use `Arc<T>` for multi-threaded shared ownership
- `Rc` has no runtime overhead beyond the count increment/decrement

---

### W2-P4 — RefCell<T> (`src/refcell.rs`)

**[Docs: RefCell\<T\>](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)**

Mutate data through an immutable (`&self`) reference using runtime borrow checking.

```rust
use std::cell::RefCell;

struct Counter { value: RefCell<i32> }

impl Counter {
    fn increment(&self) { *self.value.borrow_mut() += 1; }
    fn get(&self) -> i32 { *self.value.borrow() }
}
```

#### Compile-time vs runtime borrow checking

```
Normal Rust (compile-time):          RefCell (runtime):

fn increment(&mut self) {            fn increment(&self) {
    self.value += 1;                     *self.value.borrow_mut() += 1;
}                                    }
   ▲                                        ▲
   Compiler enforces the rule.              RefCell enforces the rule.
   Violation = compile error.              Violation = runtime panic.
   Zero overhead.                          Small overhead (tracks borrow state).
```

#### What RefCell tracks internally

```
RefCell<i32>
┌────────────────────────────────┐
│  borrow_count: 0               │  ← how many active borrow() calls
│  mut_borrow: false             │  ← is borrow_mut() active?
│  value: 42                     │  ← the actual data
└────────────────────────────────┘

borrow()      → increments borrow_count, returns Ref<i32>
borrow_mut()  → sets mut_borrow = true, returns RefMut<i32>
Drop Ref      → decrements borrow_count
Drop RefMut   → sets mut_borrow = false

PANIC if:  borrow_mut() called while borrow_count > 0
PANIC if:  borrow() or borrow_mut() called while mut_borrow = true
```

#### Key ideas

- **Interior mutability**: mutate data even when you only hold `&self` (immutable reference)
- `borrow()` → `Ref<T>` — immutable, like `&T`; multiple allowed at once
- `borrow_mut()` → `RefMut<T>` — mutable, like `&mut T`; exclusive — no other borrows active
- `Ref`/`RefMut` are dropped at end of statement (or when explicitly dropped) — borrow released
- Borrow violations → **runtime panic**, not compile error — use carefully
- Single-threaded only — use `Mutex<T>` for multi-threaded interior mutability
- **Common pattern**: `Rc<RefCell<T>>` = multiple owners + mutation in single-threaded code

---

### W2-P5 — Trait objects / Box<dyn Trait> (`src/trait_objects.rs`)

**[Docs: Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)**

Store different concrete types in the same collection via dynamic dispatch.

```rust
trait Shape { fn area(&self) -> f64; }
impl Shape for Circle { fn area(&self) -> f64 { PI * self.radius * self.radius } }
impl Shape for Rect   { fn area(&self) -> f64 { self.w * self.h } }

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
```

#### Static dispatch (generics) vs dynamic dispatch (trait objects)

```
STATIC — fn largest<T: Shape>(s: &T)       DYNAMIC — fn draw(s: &dyn Shape)
─────────────────────────────────           ──────────────────────────────────
Compiler generates one version              One function, resolved at runtime
of the function per concrete type.          via vtable lookup.

Faster at runtime (inlined).                Tiny runtime cost (one pointer follow).
Larger binary (more code generated).        Smaller binary.
Type known at compile time.                 Type NOT known at compile time.
Cannot mix types in a Vec.                  Can mix types in a Vec<Box<dyn Shape>>.
```

#### Memory layout of Vec<Box<dyn Shape>>

```
Vec on stack:
┌─────────────────────┐
│ ptr, len, cap       │
└──────────┬──────────┘
           │
           ▼  (heap — the Vec's buffer)
┌──────────────────┬──────────────────┬──────────────────┐
│  Box<dyn Shape>  │  Box<dyn Shape>  │  Box<dyn Shape>  │
│  [data_ptr]      │  [data_ptr]      │  [data_ptr]      │
│  [vtable_ptr]    │  [vtable_ptr]    │  [vtable_ptr]    │
└────────┬─────────┴────────┬─────────┴────────┬─────────┘
         │                  │                  │
         ▼                  ▼                  ▼
    Circle{r:1.0}      Rect{w:2,h:3}     Circle{r:2.0}
    (on heap)          (on heap)          (on heap)
         │                  │
         ▼                  ▼
   Circle's vtable     Rect's vtable
   [area: fn ptr]      [area: fn ptr]
```

Every `Box<dyn Shape>` is a **fat pointer**: data pointer + vtable pointer. Same size everywhere.

#### Key ideas

- `dyn Trait` = trait object — concrete type erased, method calls resolved via vtable at runtime
- `Box<dyn Trait>` is required to store trait objects — `dyn Trait` alone has no known size
- A fat pointer = (pointer to data, pointer to vtable) — always 2 × pointer size (16 bytes on 64-bit)
- Vtable contains one function pointer per trait method — calling `.area()` follows the vtable
- `&dyn Trait` also works for borrowing; `Box<dyn Trait>` is for ownership
- Use when: mixed types in a collection, plugin systems, callback-style APIs
- Use generics instead when: single type known at compile time and you want zero overhead

---

### W2-P6 — Trait object pipeline (`src/pipeline.rs`)

**[Docs: Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)**

Build a dynamic processing pipeline where each step is a different `Formatter` implementation.

```rust
trait Formatter { fn format(&self, input: &str) -> String; }

struct Upper; struct Snake; struct Trim;
// impl Formatter for each...

fn apply_all(input: &str, fmts: &[Box<dyn Formatter>]) -> String {
    fmts.iter().fold(input.to_string(), |acc, f| f.format(&acc))
}
```

#### How .fold() threads data through the pipeline

```
input: "  hello world  "

fmts:  [Trim,          Upper,          Snake        ]
        │               │               │
        ▼               ▼               ▼
acc:  "  hello world  "
        │
        Trim::format()
        │
        ▼
acc:  "hello world"
        │
        Upper::format()
        │
        ▼
acc:  "HELLO WORLD"
        │
        Snake::format()
        │
        ▼
acc:  "HELLO_WORLD"    ← final result
```

#### Key ideas

- **Unit structs** (`struct Upper;`) have no fields and take zero bytes — they exist only as a type to attach an `impl` to
- `.fold(init, |acc, item| ...)` — threads `acc` through every step; output of each becomes input to next
- The pipeline order is determined at runtime by the order of the `Vec` — swap the `Box::new(...)` calls to change behaviour
- Each `Box<dyn Formatter>` is a fat pointer: one pointer to the zero-byte unit struct (or actual data), one to the vtable
- This pattern (Strategy pattern) separates *what* to do (the `Vec`) from *how* (each `impl`)
- `.replace(' ', "_")` — `char` literal uses single quotes; string literal uses double quotes
