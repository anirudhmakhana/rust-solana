fn sum_of_squares(nums: &[i32]) -> i32 {
    nums.iter().map(|x| x * x).sum()
}

fn main() {
    println!("{}", sum_of_squares(&[1, 2, 3]));
}
