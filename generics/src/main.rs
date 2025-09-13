// fn main() {
//     let number_list = vec![23, 32, 76, 48];
//     let number_list_2 = vec![0.1, 0.3, 0.5];

//     let r = largest(&number_list);
//     let r2 = largest(&number_list_2);
//     println!("The largest number is {r2}");
// }

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut result: &T = &list[0];

//     for item in list {
//         if item > result {
//             result = item;
//         }
//     }
//     result
// }

// hwo we can use generics in the structs

// struct Point<T> {
//     x: T,
//     y: T,
// }
// fn main() {
//     let integer = Point { x: 20, y: 30 };
//     let string = Point { x: "hello", y: "hyy" };
// }

use std::{ collections::btree_set::Intersection, ffi::c_long };

//now this is how you can define dynamic types
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn create_point(x: T, y: U) -> Self {
        Self { x, y }
    }
}
fn main() {
    let integer = Point::create_point(10, 20);
    let string = Point::create_point("none", 0.5);

    println!("This is interger generic {:?}", integer);
    println!("This is mix generaic {:?}", string);
}
