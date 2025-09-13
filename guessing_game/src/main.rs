use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please Input your number: ");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read message");

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a valid Number ");
                continue;
            }
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("The number you gess is greator "),
            Ordering::Less => println!("The number you gess is Shorter "),
        }
    }
}
