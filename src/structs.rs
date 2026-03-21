struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn square(size: i32) -> Self {
        Self { width: size, height: size }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let r1 = Rectangle { width: 4, height: 5 };
    println!("area: {}", r1.area());
    println!("is_square: {}", r1.is_square());

    let r2 = Rectangle::square(6);
    println!("area: {}", r2.area());
    println!("is_square: {}", r2.is_square());
}
