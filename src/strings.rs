// Strings
// Primitive Strings = Immutable, fixed-length, in-memory
// String = Growable, heap-allocated data-structure (Use when you to modify string data)

pub fn run() {
    // Mutable Primitive String
    let mut hello = String::from("Hello");

    // Get length of string
    println!("Length: {}", hello.len());

    // Two main methods to add to a string
    // -- Push Method for chars
    // -- Push STR Method
    //
    // Attempting a push without making the string mutable will result in error
    //
    hello.push('W');
    hello.push_str("orld!");

    // NOTE: The above pushes would not work if we initialized the string like this:
    // let hello = "Hello" --> This creates a datatype of &str instead

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if string is empty
    println!("Is Empty: {}", hello.is_empty());

    println!("{}", hello)
}
