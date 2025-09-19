#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn structures() {
    let name = String::from("Kartik");
    let age = 27;

    let kartik = Person { name, age };

    // println!("{:?}", kartik)

    let point = Point { x: 4.5, y: 5.6 };

    let another_point = Point { x: 44.6, y: 65.5 };

    // println!("Point Coordinates: ({}, {})", point.x, point.y)

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 3.3, ..another_point };

    println!("Secodn point ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;
}
