// Traditional struct 
struct Color {
    red: u8,
    blue: u8,
    green: u8
}

struct TupleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
         Person {
             first_name: first.to_string(),
             last_name: last.to_string()
         }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        blue: 0,
        green: 0
    };

    c.red = 200;

    println!("Traditional Struct Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = TupleColor(255, 0, 0);
    c2.1 = 2;
    println!("Tuplet Color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Łukasz", "Gos");
    println!("Person full name before last name change: {}", p.full_name());
    p.set_last_name("Świercz");
    
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Person full name: {}", p.full_name());
    println!("Person to tuple: {:?}", p.to_tuple());
}