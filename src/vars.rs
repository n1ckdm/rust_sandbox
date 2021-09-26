// Variable hold primitive data or references to data
// Veriables are immutable by default
// Rust is a block scoped language

pub fn run() {
    let name = "Nick";
    let mut age = 35;

    println!("My name is {name} and I am {age} years old", name=name, age=age);
    age = 36;
    println!("My name is {name} and I am {age} years old", name=name, age=age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Define multiple variables at once:
    let (pokemon, number) = ("Pikachu", 33);
    println!("My favourite pokemon is {} and my faviourite number is {}", pokemon, number);
}