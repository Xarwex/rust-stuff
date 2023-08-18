use std::cell::RefCell;

fn main() {
    let x = RefCell::new(String::from("helo"));
    ref_cell_mut(&x);
    // This works normally ayyy
    println!("{:?}", x);
    ref_cell_mut_no_scam(&x);
}

fn ref_cell_mut(x: &RefCell<String>) {
    let mut reference_x = x.borrow_mut();
    *reference_x = "word".to_string();
}

fn ref_cell_mut_no_scam(x: &RefCell<String>) {
    let ref_x_1 = x.borrow_mut();
    let ref_x_2 = x.borrow_mut();
    println!("wow this surely does not panic {ref_x_1:?} {ref_x_2:?}");
}
