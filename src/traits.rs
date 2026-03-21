trait Describable {
    fn describe(&self) -> String;
}

struct Item {
    name: String,
    price: i32,
}

impl Describable for Item {
    fn describe(&self) -> String {
        format!("{}: {} cents", self.name, self.price)
    }
}

fn main() {
    let item = Item { name: String::from("Apple"), price: 150 };
    println!("{}", item.describe());
}
