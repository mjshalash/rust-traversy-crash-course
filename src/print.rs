// Public signifies a public function
// We are going to call this function from the main.rs file
pub fn run() {
    println!("Hello from the print.rs file");

    // Placeholder
    // println!(1); This would not be a valid print as you need a string literal for formatting numbers
    // Below is correct syntax, the placeholder {} will be replaced with 1
    println!("Number: {}", 1);

    // Multiple Placeholders
    println!("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    // The list of values are zero-indexed and can be referenced in placeholders
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code" );

    // Named Arguments
    // Alternative to positional where you reference values to use based on names
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Placeholder traits
    // Can utilize specific traits of placeholders
    println!("Binary: {:b} Hex {:x} Octal {:o}", 10, 10, 10);

    // Placeholder for debug trait
    // Can specify a tuple containing multiple types to get substituted in for a :? placeholder
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}