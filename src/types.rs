// Datatypes in Rust

/**
 * Primitive Types
 * -Integers: u8, i8, u16, i16, u32, i32, u64m i64, u128, i128 (Number of bits in memory)
 *  -- u = unsigned
 * - Floats: f32, f64
 * - Boolean,
 * - Chars
 * - Tuples
 * - Arrays
 * -- Vectors
 *
 * Rust is statically typed, thus it needs to know the data types of all variables at compile time.
 * Compiler CAN infer datatypes without explicit declaration based on the value and how it is used
 * */

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Specifying explicit datatype
    let z: i64 = 45454454545454;

    // Find Max size of datatype
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    // Note: Convention for Rust variables is snake_case, not camelCase
    let is_active = true;

    // Getting boolean from an expression
    let is_greater: bool = 10 < 5;

    // CHAR
    // let a1 = 'ab' --> This would give you an error since the single-quotes signify a single character
    let a1 = 'a';

    // Can also print emojis with char datatype using their respective unicodes
    let face = '\u{1F600}';

    // Printing a tuple
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
