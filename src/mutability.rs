fn sum_to(n: i32) -> i32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

fn main() {
    println!("{}", sum_to(10));
}
