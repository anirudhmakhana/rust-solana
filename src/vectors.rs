fn sum_vec(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{}", sum_vec(&v));
}
