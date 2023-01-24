// https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md

use std::mem::size_of;

fn main() {
    // primitives
    assert_eq!(4, size_of::<i32>());
    assert_eq!(8, size_of::<f64>());

    // tuples
    assert_eq!(8, size_of::<(i32, i32)>());

    // arrays
    assert_eq!(0, size_of::<[i32; 0]>());
    assert_eq!(12, size_of::<[i32; 3]>());

    struct Point {
        _x: i32,
        _y: i32,
    }

    // structs
    assert_eq!(8, size_of::<Point>());

    // enums
    assert_eq!(8, size_of::<Option<i32>>());

    // get pointer width, will be
    // 4 bytes wide on 32-bit targets or
    // 8 bytes wide on 64-bit targets
    const WIDTH: usize = size_of::<&()>();

    // pointers to sized types are 1 width
    assert_eq!(WIDTH, size_of::<&i32>());
    assert_eq!(WIDTH, size_of::<&mut i32>());
    assert_eq!(WIDTH, size_of::<Box<i32>>());
    assert_eq!(WIDTH, size_of::<fn(i32) -> i32>());

    const DOUBLE_WIDTH: usize = 2 * WIDTH;

    // unsized struct
    #[derive(Debug)]
    struct Unsized {
        _unsized_field: [i32],
    }

    // impl Unsized {
    //     pub fn new(slice: &[i32]) -> &Unsized {
    //         unsafe { &*(slice as *const [i32] as *const Unsized) }
    //     }
    // }

    // pointers to unsized types are 2 widths
    assert_eq!(DOUBLE_WIDTH, size_of::<&str>()); // slice
    assert_eq!(DOUBLE_WIDTH, size_of::<&[i32]>()); // slice
    assert_eq!(DOUBLE_WIDTH, size_of::<&dyn ToString>()); // trait object
    assert_eq!(DOUBLE_WIDTH, size_of::<Box<dyn ToString>>()); // trait object
    assert_eq!(DOUBLE_WIDTH, size_of::<&Unsized>()); // user-defined unsized type

    // unsized types
    // size_of::<str>(); // compile error
    // size_of::<[i32]>(); // compile error
    // size_of::<dyn ToString>(); // compile error
    // size_of::<Unsized>(); // compile error

    // struct UnsizedWrong {
    //     _unsized_field_2: [i32],
    //     _unsized_field_1: [i32],
    // }
}

/*
- slices are double-width because they store a pointer to the array and the number of elements in the array
- trait object pointers are double-width because they store a pointer to the data and a pointer to a vtable
- unsized structs pointers are double-width because they store a pointer to the struct data and the size of the struct
- unsized structs can only have 1 unsized field and it must be the last field in the struct
*/
