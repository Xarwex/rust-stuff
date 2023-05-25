use std::marker::PhantomData;

struct Crab<T> {
    _x: PhantomData<T>,
}

impl<T> Crab<T> {}

fn main() {
    let _x: Vec<Crab<u64>> = vec![];
}
