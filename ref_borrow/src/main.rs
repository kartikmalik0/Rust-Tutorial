// fn main() {
//     let name = String::from("Kartik");
//     let name_length = calculate_length(&name);
//     println!("This is length of {} is {}", { name }, { name_length });
// }

// fn calculate_length(name: &String) -> usize {
//     let length = name.len();
//     length
// }

// now let's look at how we can define the mutable refrences

fn main() {
    let mut name = String::from("Kartik");
    let name_length = calculate_length(&mut name);
    println!("This is length of {} is {}", { name }, { name_length });
}

fn calculate_length(name: &mut String) -> usize {
    name.push_str("Hello");
    let length = name.len();
    length
}
