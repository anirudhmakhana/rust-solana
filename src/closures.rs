fn double_all(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|x| x * 2).collect()
}

fn main() {
    println!("{:?}", double_all(&[1, 2, 3, 4, 5]));
}
