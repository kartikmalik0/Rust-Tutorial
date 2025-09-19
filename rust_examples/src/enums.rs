#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}
pub fn enums() {
    // Equivalent to `Stage::Beginner`.
    let stage = Stage::Beginner;
    // Equivalent to `Role::Student`.
    let role = Role::Teacher;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Stage::Beginner => println!("Beginners are starting their learning journey!"),
        Stage::Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Role::Student => println!("Students are acquiring knowledge!"),
        Role::Teacher => println!("Teachers are spreading knowledge!"),
    }
}
