// Structs are used to create custom data types

use std::fmt;

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Implement a debug function
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Color")
         .field("r", &self.red)
         .field("g", &self.green)
         .field("b", &self.blue)
         .finish()
    }
}

struct Person {
    firstname: String,
    lastname: String
}

impl Person {
    fn new(firstname: &str, lastname: &str) -> Person {
        Person {
            firstname: firstname.to_string(),
            lastname: lastname.to_string()
        }
    }

    fn fullname(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }

    fn set_lastname(&mut self, lastname: &str) {
        self.lastname = lastname.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.firstname, self.lastname)
    }
}

pub fn run() {

    let mut c = Color{
        red: 255, green: 10, blue: 50
    };

    println!("{:?}", c);
    c.blue = 100;
    println!("{:?}", c);

    let mut nick = Person::new("Nick", "Martin");
    println!("Person: {}", nick.fullname());
    nick.set_lastname("Awesome");
    println!("Person: {}", nick.fullname());
    println!("Person as tuple {:?}", nick.to_tuple());
    println!("Person: {}", nick.fullname());
}