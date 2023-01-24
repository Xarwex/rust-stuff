// https://doc.rust-lang.org/reference/items/traits.html

use std::mem::size_of;

trait ObjectSafe {
    fn crab(&self);
}

// The pointer to this trait will be single width and we need two pointers - second one will point to the vtable

// // trait NonObjectSafe: Sized {}
// trait NonObjectSafe {
//     fn crab(&self) -> Self;
//     fn is();
//     fn righ<T>(&self, x: T) {}
// }

// struct ThisWontWork {
//     object: dyn NonObjectSafe,
// }

trait ObjectSafeWithNonDispatch {
    fn sized_crab(&self)
    where
        Self: Sized;
}

#[allow(dead_code)]
struct Strategy {
    object1: Box<dyn ObjectSafe>,
    object2: Box<dyn ObjectSafeWithNonDispatch>,
}

#[allow(dead_code)]
impl Strategy {
    fn make_money(&self) {
        self.object1.crab();
        // self.object2.sized_crab();
    }
}

fn main() {
    assert_eq!(16, size_of::<&dyn ObjectSafe>());
    assert_eq!(16, size_of::<&dyn ObjectSafeWithNonDispatch>());
    // assert_eq!(8, size_of::<&dyn NonObjectSafe>()); // would have to be the case but this is not possible :((
}
