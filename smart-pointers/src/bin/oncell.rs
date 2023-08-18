use std::sync::OnceLock;

static MY_VAL: OnceLock<u64> = OnceLock::new();

fn main() {
    println!("{:?}", MY_VAL.get());
    println!("{:?}", MY_VAL.set(10));
    println!("{:?}", MY_VAL.get());
    println!("{:?}", MY_VAL.set(11));
}
