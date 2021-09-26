// Enums are types which have some definite values

struct Person {
    name: String,
    pos_x: i32,
    pos_y: i32
}

enum Movement {
    Up,
    Down,
    Left,
    Right
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
            pos_x: 0,
            pos_y: 0
        }
    }

    fn move_person(&mut self, m: Movement) {
        // match is similar to switch
        match m {
            Movement::Up => { self.pos_y += -1; println!("Moving up!") },
            Movement::Down => { self.pos_y += 1; println!("Moving down!") },
            Movement::Left => { self.pos_x += -1; println!("Moving left!") },
            Movement::Right => { self.pos_x += 1; println!("Moving right!") },
        }
    }

    fn position(&self) -> String {
        format!("({},{})", self.pos_x, self.pos_y)
    }
}

impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Person")
         .field("name", &self.name)
         .field("pos", &self.position())
         .finish()
    }
}



pub fn run() {
    let mut person = Person::new("Nick");

    println!("{}", person.position());

    println!("{:?}", person);

    person.move_person(Movement::Up);
    person.move_person(Movement::Left);
    person.move_person(Movement::Up);

    println!("{:?}", person);

    person.move_person(Movement::Down);
    person.move_person(Movement::Down);
    person.move_person(Movement::Down);
    person.move_person(Movement::Down);
    person.move_person(Movement::Down);
    person.move_person(Movement::Right);
    person.move_person(Movement::Right);
    person.move_person(Movement::Right);

    println!("{:?}", person);
}