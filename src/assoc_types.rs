// NOTE: Associated type vs generic parameter — the key difference:
//
//   Generic:  trait Summary<T> { fn summarize(&self) -> T; }
//             A type could impl Summary<i32> AND Summary<String> — ambiguous at call sites.
//
//   Associated: trait Summary { type Output; fn summarize(&self) -> Self::Output; }
//             A type can only impl Summary ONCE — Output is fixed per type.
//             Call sites don't need to specify the output type — the compiler knows it.
trait Summary {
    type Output;
    fn summarize(&self) -> Self::Output;
}

struct Numbers {
    data: Vec<i32>,
}

struct Words {
    data: Vec<String>,
}

// NOTE: For Numbers, we fix Output = i32.
// The compiler now knows: Numbers::summarize() always returns i32.
// No annotation needed at the call site.
impl Summary for Numbers {
    type Output = i32;

    fn summarize(&self) -> i32 {
        // NOTE: iter().sum() works because i32 implements the Sum trait.
        // The return type annotation on the fn tells the compiler what sum() should produce.
        self.data.iter().sum()
    }
}

// NOTE: For Words, Output = String — a completely different type from Numbers.
// Same trait, different associated type — this is the power of associated types.
impl Summary for Words {
    type Output = String;

    fn summarize(&self) -> String {
        // NOTE: join() takes a slice of Strings and concatenates them with a separator.
        // self.data is Vec<String>; join works on &[String] — auto-deref from &Vec<String>.
        self.data.join(" ")
    }
}

fn main() {
    let nums = Numbers { data: vec![1, 2, 3, 4, 5] };
    // NOTE: No type annotation needed — compiler knows nums.summarize() returns i32.
    println!("{}", nums.summarize()); // 15

    let words = Words {
        data: vec!["hello".to_string(), "from".to_string(), "rust".to_string()],
    };
    // NOTE: Compiler knows words.summarize() returns String.
    println!("{}", words.summarize()); // hello from rust
}
