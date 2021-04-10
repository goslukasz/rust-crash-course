pub fn run() {
    let age = 20;
    let check_id: bool = false;

    if age >= 21 && check_id {
        println!("Bartender: What would you like to dring?");
    } else {
        println!("Bartender: Nope");
    }

    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Shorthand: {}", is_of_age);
}