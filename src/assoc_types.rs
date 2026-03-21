trait Summarize {
    type Output;
    fn summarize(&self) -> Self::Output;
}

struct Numbers {
    data: Vec<i32>,
}

struct Sentence {
    words: Vec<String>,
}

impl Summarize for Numbers {
    type Output = i32;
    fn summarize(&self) -> i32 {
        self.data.iter().sum()
    }
}

impl Summarize for Sentence {
    type Output = String;
    fn summarize(&self) -> String {
        self.words.join(" ")
    }
}

fn main() {
    let nums = Numbers { data: vec![1, 2, 3, 4, 5] };
    println!("{}", nums.summarize());

    let sentence = Sentence {
        words: vec!["hello".to_string(), "from".to_string(), "rust".to_string()],
    };
    println!("{}", sentence.summarize());
}
