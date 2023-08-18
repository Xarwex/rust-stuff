#![feature(cow_is_borrowed)]
use std::borrow::Cow;

fn abs_all(input: &mut Cow<'_, [i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
}

fn main() {
    cowge(&[1, 2, 3]);
    cowge(&[-1, 2, 3]);
}

fn cowge(slice: &[i32]) {
    println!("Input: {slice:?}");
    let mut cow = Cow::from(slice);
    abs_all(&mut cow);
    println!("Output: {cow:?}, is borrowed - {:?}", cow.is_borrowed());
}
