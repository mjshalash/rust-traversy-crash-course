// Tuples are used to group together elements of different types
// Tuples have a max size of 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Malik", "NY", 45);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
