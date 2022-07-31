// Variables hold primitive data or references to data
// Rust is Block-Scoped
// Variables are immutable, by default

pub fn run() {
    let name = "Brad";

    // This will result in an error as, by default, age variable is not mutable
    // Need to utilize the mut keyword
    // let age = 37;
    // age = 38;

    // This is valid, although, you will get a warning that you never used the original value of 37
    let mut age = 37;
    age = 38;

    println!("My name is {} and I am {}", name, age);

    // const keyword
    // Must define a type with const usage
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign Multiple Variables at one time
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
