use std::rc::Rc;

#[tokio::main]
async fn main() {
    let x = Rc::new(10);
    let y = 10;
    let _res = tokio::join! {
        tokio::spawn(async move {
            // x;
            println!("{y}");
        }),
        // tokio::spawn(rc_illegal(x))
    };
}

async fn rc_illegal(_x: Rc<i32>) {}
