// fn main() {
//     println!("Hello, world!");

//     let s = "this is s from the main scope";

//     {
//         let x = "this is the x from the small scope"; // under the rule of ownership we cannot access this variable outside of the owner (scope) it got killed with its owner
//     }

//     // println!("Value form x {}", x);
//     println!("Value form s {}", s);

//     let mut expandable_string = String::from("Hello"); // You can expand this type and its stored in heap memory not in the stach and we can push things here

//     expandable_string.push_str(" World");

//     println!("{}", expandable_string)
// }

// owner ship in functions

// fn main() {
//     let num = 15;

//     let name = String::from("Kartik Malik"); // store the value in the heap

//     transfer_ownership(name);
//     let s = gives_ownership();
//     println!("Now this s has the ownership {}", s);
//     let s2 = takes_and_gives_back_ownership(s);
//     // now we cannot access s1 its ownership transferred to the s2 now s2 is the new owner

//     println!("{s2}")
//     // println!("Value of name is {name}"); // so we can't use it here after the ownership trnasferred
// }

// fn transfer_ownership(s: String) {
//     println!("Inside Ownership {s}")
// }

// fn gives_ownership() -> String {
//     let s = String::from("This funtion gives ownership");
//     s
// }

// fn takes_and_gives_back_ownership(s: String) -> String {
//     println!("This funtion gives and tranfer ownership {s}");
//     s
// }

// now what if we wnat to pass the value but don't let them take the ownership

fn main() {
    let name = String::from("Kartik");
    let (name, leng) = calculate_length(name);
    println!("The lenght {name} is {leng}")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
