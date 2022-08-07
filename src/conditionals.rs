// Conditionals - Check condition of something and then execute portions of the program accordingly
// Very standard usage of conditionals, very similar to other languages

pub fn run() {
    let age: u8 = 30;
    let check_id: bool = true;

    // If/Else
    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };

    println!("Is Of Age: {}", is_of_age);
}
