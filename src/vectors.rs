// Vectors are resizeable arrays
// Will probably use vectors more than Arrays

use core::num;
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value from vector
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // With mutable arrays, because you are not adding the value in but rather re-assigning
    numbers[2] = 20;

    // Can push onto vectors
    numbers.push(6);
    numbers.push(7);

    // Can also pop the elements off of vectors
    numbers.pop();

    println!("{:?}", numbers);

    // Memory of an Array
    // Notice, we are passing in a reference to the array of numbers that we want to check, not the values themselves
    //println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Side Note: We do not have to include std:: portion if we include it at the top
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Slices
    // Can associate a slice with a reference for a data structure to obtain various "slices" of said structure
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);

    // Looping through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    // Note the usage of iter_mut instead of iter
    // Similar to the .map() method in javascript
    for y in numbers.iter_mut() {
        *y *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
