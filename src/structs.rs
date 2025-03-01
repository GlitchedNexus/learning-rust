// Structs - Used to create custom data types

// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

struct Person {
    first_name: String,
    last_name: String,
  }
  
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
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

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// Tuple Struct
struct Colour(u8, u8, u8);

pub fn run() {
    let c1 = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Color: {} {} {}", c1.red, c1.green, c1.blue);

    let c2 = Colour(255, 0, 0);

    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Mary", "Doe");

    println!("Person {}", p.full_name());

    p.set_last_name("Williams");

    println!("Person {}", p.full_name());
    
    println!("Person Tuple {:?}", p.to_tuple());
}