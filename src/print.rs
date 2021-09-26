pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is {} years old", "Nick", 35);

    // Positional arguments
    println!("{0} is {1} years old and {0} loves rust", "Nick", 35);

    // Named arguments
    println!("{name} is {age} years old", name = "Nick", age = 35);

    // Placeholder traits
    println!("Binary: {0:b} | Hex: {0:x} | Octal: {0:o}", 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "help"));

    // Basic maths
    println!("10 + 10 = {}", 10 + 10);
}