fn parse_csv_sum(s: &str) -> Result<i32, String> {
    if s.is_empty() {
        return Err("empty input".to_string());
    }

    let mut sum = 0;
    for token in s.split(',') {
        let n = token
            .trim()
            .parse::<i32>()
            .map_err(|_| format!("invalid number: {}", token.trim()))?;
        sum += n;
    }
    Ok(sum)
}

fn main() {
    println!("{:?}", parse_csv_sum("1,2,3"));
    println!("{:?}", parse_csv_sum(""));
    println!("{:?}", parse_csv_sum("1,two,3"));
}
