fn add_parsed(a: &str, b: &str) -> Result<i32, String> {
    let a = a.parse::<i32>().map_err(|_| "parse error".to_string())?;
    let b = b.parse::<i32>().map_err(|_| "parse error".to_string())?;
    Ok(a + b)
}

fn main() {
    println!("{:?}", add_parsed("10", "20"));
    println!("{:?}", add_parsed("10", "abc"));
}
