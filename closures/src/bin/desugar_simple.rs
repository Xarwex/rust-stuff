struct FStruct<'a> {
    a: &'a f64,
    b: &'a f64,
}

impl FStruct<'_> {
    fn call(&self, x: f64) -> f64 {
        self.a * x + self.b
    }
}

fn main() {
    let x = 3.0;
    println!("f({}) = {}", x, FStruct { a: &2.0, b: &2.0 }.call(x));
}
