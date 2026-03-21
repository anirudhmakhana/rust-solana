fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &n in &list[1..] {
        if n > max {
            max = n;
        }
    }
    max
}

fn main() {
    println!("{}", largest(&[3, 7, 1, 9, 4]));
}
