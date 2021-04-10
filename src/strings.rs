pub fn run() {
    //Static string: immutable
    let hello = "Hello";

    //Dynamic string: mutable
    let mut hello2 = String::from("Hello ");
    hello2.push('W');
    hello2.push_str("orld! ");
    hello2.push('\u{1F600}');

    println!("Immutable: {}, Mutable: {}", hello, hello2);
    println!("Length: {}", hello2.len());

    //Capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    //Is empty
    println!("Is empty: {}", hello2.is_empty());

    //Contains
    println!("Contains 'World': {}", hello2.contains("World"));

    //Replace 
    println!("Replace: {}", hello2.replace("World", "Home"));

    //Loop through stringby whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10); 
    s.push('a');
    s.push('b');

    println!("Capacity string: {}", s);

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}