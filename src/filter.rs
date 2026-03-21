fn evens_only(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

fn main() {
    println!("{:?}", evens_only(&[1, 2, 3, 4, 5, 6]));
}
