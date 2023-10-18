trait Bonk1<T> {
    fn bonk(&self);
}

trait Bonk2 {
    fn bonkbonk(&self);
}

impl<T> Bonk1<T> for dyn Bonk2 {
    fn bonk(&self) {
        self.bonkbonk()
    }
}

impl<T> Bonk2 for dyn Bonk1<T> {
    fn bonkbonk(&self) {}
}

fn main() {
    println!("Hello, world!");
}

struct MyStruct {}

mod jerrys_thing_layer {

    use crate::MyStruct;
    trait JerrysTrait {}
    // impl JerrysTrait for dyn WrapperType {}
    // impl WrapperType for MyStruct {}
    impl JerrysTrait for MyStruct {}
}
