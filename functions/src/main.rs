fn main() {
    // println!("Hello, world!");
    // let y = true;

    // let number = if y { 10 } else { 20 };
    // println!("Number is: {}", number);
    // my_function(number)

    let mut num = 0;

    loop {
        println!("This is the value of Number: {num}");

        if num == 10 {
            break;
        }
        num = num + 1;
    }

    println!("This is the end")
}

// fn my_function(num: i32) {
//     if num % 4 == 0 {
//         println!("Number is divisible by 4")
//     } else if num % 3 == 0 {
//         println!("Number is divisible by 3")
//     } else if num % 2 == 0 {
//         println!("Number is divisbile by 2")
//     } else {
//         println!("Number is divisible by 4, 3, 2")
//     }
// }
