// struct Ms {
//     closure: Box<dyn Fn(u64) -> u64>,
// }

fn main() {
    let a = "hello".to_string();
    let fs = || {
        let b = a;
        println!("we got {}", b);
    };
    fs();

    let mut a = "hello".to_string();
    // Mutable binding requires this to be mutable
    let mut fs = move || {
        a.push_str(" world");
        println!("{a}");
    };

    fs();
    // fs();
    // println!("{a}");

    // let f = |x: u64| x;

    // let c = Ms {
    //     closure: Box::new(f),
    // };
}
