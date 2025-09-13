// fn main() {
// let mut v = Vec::new();

// v.push(1);
// v.push(2);
// v.push(3);
// v.push(4);

// println!("This is the v {:?}", v);

// let mut v = vec![1, 2, 3, 4];

// how to get the v value of an particular index

//let second_value = v.get(1).unwrap_or(&-1); // direct unwrap will crash the program if we will use some uncommon index and unwrap_or will provide a default value in case program fails

//  and other way to handle is match

// let second_value = match v.get(34) {
//     Some(val) => val,
//     None => {
//         println!("Please provide a valid index");
//         &-1
//     }
// };

// println!("This is  the v {:?} and this is its second index value {second_value}", v)

// now lets loop over the vector

// In the loop for i in &mut v, -----------------
// i is not the number, it’s a pointer (reference) to the number.

// To change the number, you need to follow the pointer.
// In Rust we do that with
// for i in &mut v {
//     println!("i is {:?}", i);
//     *i = *i * 2;
// }
// print!("{:?}", v)

// assiging mulitple types to vecor using enums--------------

// #[derive(Debug)]
// enum SpreadSheetCell {
//     Int(i32),
//     Float(i16),
//     String(String),
// }

// let cell: Vec<SpreadSheetCell> = vec![
//     SpreadSheetCell::Int(20),
//     SpreadSheetCell::Float(30),
//     SpreadSheetCell::String(String::from("Hello")),
//     SpreadSheetCell::Int(30),
//     SpreadSheetCell::Float(70),
//     SpreadSheetCell::String(String::from("Hello_2"))
// ];

// match cell.get(2) {
//     Some(SpreadSheetCell::Int(val)) => {
//         println!("This is the int {}", val);
//     }
//     Some(val) => {
//         println!("This is some other value {:?}", val);
//     }
//     None => {
//         print!("This is wrong index");
//     }
// }

// print!("This is how are mix vecotr looks like using a enum = {:#?}", cell)
// }

use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&'static str, i32> = HashMap::new();

    scores.insert("blue", 10);
    scores.insert("yellow", 20);

    let score = scores.get("blue").unwrap_or(&0);
    println!("Blue team score: {}", score);
}
