fn main() {
    let r: i32 = match divide(4, 0) {
        Ok(num) => num,
        Err(err) => {
            println!("Error {err}");
            -1
        }
    };
    println!("R = {r}")
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Please dont not divide by zero"));
    }

    Ok(x / y)
}
