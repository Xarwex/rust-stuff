#[tokio::main]
async fn main() {
    let x = 10;
    let ref_x: &'static u64 = Box::leak(Box::new(x));
    let _res = tokio::join! {
        tokio::spawn(show_x(ref_x)),
        tokio::spawn(show_x(ref_x))
    };
}

async fn show_x(x: &u64) {
    println!("{x}");
}
