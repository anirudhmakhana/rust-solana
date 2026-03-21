fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| "invalid number".to_string())
}

fn main() {
    println!("{:?}", parse_number("42"));
    println!("{:?}", parse_number("abc"));
}
