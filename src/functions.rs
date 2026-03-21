fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= c {
        b
    } else {
        c
    }
}

fn main() {
    println!("{}", max_of_three(3, 7, 5));
}
