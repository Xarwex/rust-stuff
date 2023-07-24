use std::cell::Cell;

#[tokio::main]
async fn main() {
    let x = Cell::new(10);
    let ref_x: &'static Cell<u64> = Box::leak(Box::new(x));
    let _res = tokio::join! {
        // tokio::spawn(show_x(ref_x)),
        // tokio::spawn(show_x(ref_x))
    };
}

async fn show_x(x: &Cell<u64>) {
    x.set(5);
    println!("{x:?}");
}
