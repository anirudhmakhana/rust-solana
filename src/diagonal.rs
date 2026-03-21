fn diagonal_sum(input: &str) -> i32 {
    let matrix: Vec<Vec<i32>> = input
        .split(';')
        .map(|row| {
            row.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let n = matrix.len();
    let mut sum = 0;

    for i in 0..n {
        sum += matrix[i][i];         // primary diagonal
        sum += matrix[i][n - 1 - i]; // secondary diagonal
    }

    if n % 2 == 1 {
        let mid = n / 2;
        sum -= matrix[mid][mid]; // center counted twice
    }

    sum
}

fn main() {
    println!("{}", diagonal_sum("1 2 3;4 5 6;7 8 9"));
    println!("{}", diagonal_sum("1 2;3 4"));
}
