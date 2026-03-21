fn first_last(nums: &[i32]) -> (i32, i32) {
    (nums[0], nums[nums.len() - 1])
}

fn main() {
    let data = [10, 20, 30, 40, 50];
    let (first, last) = first_last(&data);
    println!("({}, {})", first, last);
}
