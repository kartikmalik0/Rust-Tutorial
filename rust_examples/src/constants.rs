static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

pub fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn constants() {
    let n = 16;
    println!("This is {}", LANGUAGE);
        
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" })
}
