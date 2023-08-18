use std::cell::Cell;

fn main() {
    // let x = &mut 1;
    // let y = Cell::new(1);
    // normal_mut(x);
    // println!("normal mut {}", x);
    // cell_mut(&y);
    // println!("cell mut {:?}", y);

    let z = Cell::new(String::from("hehe"));
    non_copy_mut(&z);

    // Doesn't work!
    // println!("non copy mut {:?}", z);

    println!("non_copy_mut {:?}", z.take());
}

fn normal_mut(num: &mut i32) {
    *num += 10;
}

fn cell_mut(num: &Cell<i32>) {
    let inner = num.get();
    num.set(inner + 12);
}

fn non_copy_mut(x: &Cell<String>) {
    let inner = x.take();
    x.set(inner + " bonk")
}
