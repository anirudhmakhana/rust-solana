// Expression Trees with Box, Enums, and Recursion
// Run: cargo run --bin expr_tree

// NOTE: Box<Expr> is required here — without it the enum would be infinitely sized.
// Rust needs to know the size of each variant at compile time; Box is a fixed-size pointer.
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        // NOTE: Base case — just unwrap the value.
        Expr::Num(n) => *n,

        // NOTE: Recursive cases — eval each subtree, then combine.
        // The borrow `&**l` deref-chains: Box<Expr> → Expr → &Expr.
        // Rust auto-derefs in match, so `l` here is already &Expr via the box.
        Expr::Add(l, r) => eval(l) + eval(r),
        Expr::Mul(l, r) => eval(l) * eval(r),
        Expr::Neg(e)    => -eval(e),
    }
}

fn main() {
    // (2 + 3) * -(4)  →  5 * -4  =  -20
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(2.0)),
            Box::new(Expr::Num(3.0)),
        )),
        Box::new(Expr::Neg(
            Box::new(Expr::Num(4.0)),
        )),
    );

    println!("{}", eval(&expr)); // -20
    println!("{}", eval(&Expr::Num(7.0))); // 7
    println!("{}", eval(&Expr::Neg(Box::new(Expr::Num(3.0))))); // -3
}
