// fn main() {
//     let s = String::from("Hello World");

//     let hello = &s[0..5];
//     let world = &s[6..11];

//     println!("Hello {hello}");
//     println!("Hello {world}");
// }

//strucst

struct User {
    email: String,
    name: String,
    age: u64,
    acitve: bool,
}

fn main() {
    let user_1: User = User {
        email: String::from("hello@gmail.com"),
        name: String::from("Kartik"),
        age: 23,
        acitve: false,
    };

    println!("The name of user is {}", user_1.name)
}
