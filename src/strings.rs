// Strings
// Primitive Strings = Immutable, fixed-length, in-memory
// String = Growable, heap-allocated data-structure (Use when you to modify string data)

pub fn run() {
    // Mutable Primitive String
    let mut hello = String::from("Hello");

    // Get length of string
    println!("Length: {}", hello.len());

    /* Two main methods to add to a string
     * -- Push Method for chars
     * -- Push STR Method
     *
     * Attempting a push without making the string mutable will result in error
     */
    hello.push('W');
    hello.push_str("orld!");

    // NOTE: The above pushes would not work if we initialized the string like this:
    // let hello = "Hello" --> This creates a datatype of &str instead

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if string is empty
    println!("Is Empty: {}", hello.is_empty());

    // Check if contains some substring?
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace a string or substring
    println!("Replace: {}", hello.replace("World", " There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    /*
     *   Remember, some of this stuff may seem odd in what you can do with dattypes, but Rust is for systems programming
     *    It has a different overall purpose than a language like Javascript
     */
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    /* Assertion Testing
     *   Check to see if certain qualities of the string are true
     *   This will only present a result if the assertion fails
     */
    assert_eq!(2, s.len()); // This does not present output, since it succeeds
    assert_eq!(11, s.capacity()); // This does present output since it fails
}
