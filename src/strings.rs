// Primitive strings are immutable and fixed lenght
// 'String' type is heap-allocated and modifiable

pub fn run() {
    // Primitive string:
    let primitive = "hello";

    let mut string = String::from("hello");

    println!("Lenght of {}: {}", primitive, primitive.len());
    println!("Lenght of {}: {}", string, string.len());

    // Add a character to our string:
    string.push('!');
    println!("Lenght of {}: {}", string, string.len());

    // Add a string to our string:
    string.push_str(" world");
    println!("Lenght of {}: {}", string, string.len());

    // Lots of other standard functions:
    println!("{:?}", string.contains("world"));
    println!("{:?}", string.replace("d", "d!"));

    // Write assertions:
    assert_eq!(12, string.len());
}