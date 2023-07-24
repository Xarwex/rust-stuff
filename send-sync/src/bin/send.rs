use std::sync::Arc;

#[derive(Debug)]
struct Bonk {}

#[tokio::main]
async fn main() {
    // let x = Bonk { _x: 10 };
    let x = 10;
    let box_x = Box::new(x.clone());
    let box_x = Box::new(Bonk {});
    let arc_x = Arc::new(x);
    let _res = tokio::join! {
        tokio::spawn(show_x_box(box_x.clone())),
        tokio::spawn(show_x_box(box_x)),
        tokio::spawn(show_x_arc(arc_x.clone())),
        tokio::spawn(show_x_arc(arc_x)),
    };
}

async fn show_x_box<T: std::fmt::Debug>(x: Box<T>) {
    println!("{x:?}");
}

async fn show_x_arc<T: std::fmt::Debug>(x: Arc<T>) {
    println!("{x:?}");
}
