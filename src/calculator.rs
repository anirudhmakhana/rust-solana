fn evaluate(expr: &str) -> Result<i32, String> {
    let parts: Vec<&str> = expr.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("invalid expression".to_string());
    }

    let a = parts[0].parse::<i32>().map_err(|_| "invalid expression".to_string())?;
    let op = parts[1];
    let b = parts[2].parse::<i32>().map_err(|_| "invalid expression".to_string())?;

    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0 {
                Err("division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err("unknown operator".to_string()),
    }
}

fn main() {
    println!("{:?}", evaluate("10 + 5"));
    println!("{:?}", evaluate("10 - 3"));
    println!("{:?}", evaluate("4 * 6"));
    println!("{:?}", evaluate("10 / 2"));
    println!("{:?}", evaluate("10 / 0"));
    println!("{:?}", evaluate("10 % 3"));
    println!("{:?}", evaluate("bad input"));
}
