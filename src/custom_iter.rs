struct Countdown {
    n: i32,
}

impl Countdown {
    fn new(n: i32) -> Self {
        Countdown { n }
    }
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let current = self.n;
            self.n -= 1;
            Some(current)
        } else {
            None
        }
    }
}

fn main() {
    let countdown: Vec<i32> = Countdown::new(3).collect();
    println!("{:?}", countdown);

    for n in Countdown::new(5) {
        print!("{} ", n);
    }
    println!();
}
