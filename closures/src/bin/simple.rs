fn main() {
    let x = 2.0;
    let a = 1.0;
    let b = 2.0;

    let f = |x| a * x + b;
    // If we remove this line then the type inference stops working
    println!("f({}) = {}", x, f(x));
}
