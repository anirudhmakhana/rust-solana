fn double_all(nums: &mut Vec<i32>) {
    for n in nums.iter_mut() {
        *n *= 2;
    }
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    double_all(&mut data);
    println!("{:?}", data);
}
