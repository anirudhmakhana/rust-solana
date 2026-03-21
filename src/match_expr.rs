fn coin_value(coin: &str) -> i32 {
    match coin {
        "penny" => 1,
        "nickel" => 5,
        "dime" => 10,
        "quarter" => 25,
        _ => 0,
    }
}

fn main() {
    for coin in ["penny", "nickel", "dime", "quarter", "dollar"] {
        println!("{}: {}", coin, coin_value(coin));
    }
}
