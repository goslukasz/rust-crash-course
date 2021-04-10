// Touples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Łukasz", "Gos", 37);

    println!("{:?}", person);
    println!("{} {} has {}", person.0, person.1, person.2);
}