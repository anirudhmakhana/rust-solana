fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

fn main() {
    println!("{}", factorial(0));
    println!("{}", factorial(5));
    println!("{}", factorial(10));
}
