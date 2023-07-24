use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // let x = Arc::new(RefCell::new(10));
    let y = Arc::new(10);
    // let z = Arc::new(Mutex::new(Cell::new(10)));
    let _res = tokio::join! {
        // tokio::spawn(show_x(x)),
        tokio::spawn(mut_x_arc_box(y.clone())),
        // tokio::spawn(mut_x_mutex(z)),
    };
}

async fn show_x(x: Arc<RefCell<u64>>) {
    println!("{x:?}");
}

async fn mut_x_arc_box(mut x: Arc<u64>) {
    let x = Arc::get_mut(&mut x).unwrap();
    *x += 1;
    println!("{x:?}");
}

async fn mut_x_mutex(x: Arc<Mutex<Cell<u64>>>) {
    println!("{x:?}");
}
