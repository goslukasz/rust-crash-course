pub fn run() {
    //Print
    println!("Hello from the print.rs file!");

    //Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Brad", "Mass");

    //Positional arguments
    println!("{0} is from {1} and {0} likes {2}", "Brad", "Mass", "code");

    //Named arguments
    println!("{name} like to play {activity}", name = "John", activity = "Baseball");

    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}