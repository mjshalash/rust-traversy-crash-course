// Arrays in Rust are offixed length with elements of same, fixed datatype

use std::mem;

pub fn run() {
    // [i32 = datatype of array, 5 = length]
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // :? will print out the whole thing
    // If we were to change one of the elements to a different datatype, this would fail
    // They have to be ints
    println!("{:?}", numbers);

    // Get single value from array
    println!("Single Value: {}", numbers[0]);

    // Arrays can be made mutable
    let mut mutable_numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // With mutable arrays, because you are not adding the value in but rather re-assigning
    mutable_numbers[2] = 20;

    println!("{:?}", mutable_numbers);

    // Memory of an Array
    // Notice, we are passing in a reference to the array of numbers that we want to check, not the values themselves
    //println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Side Note: We do not have to include std:: portion if we include it at the top
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Slices
    // Can associate a slice with a reference for a data structure to obtain various "slices" of said structure
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);
}
