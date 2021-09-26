/*
Primitive types...
Integers: u8-128, i8-128
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays

Rust is statically typed, but will infer most types on assignment
*/

pub fn run() {
    // Default int is 'i32'
    let _x = 1;

    // Default float is 'f64'
    let _y = 2.5;

    // We can set types explicitly with a colon:
    let _z: i64 = 123;

    // Max numbers:
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let _test = false;

    // Characters are defined using single quotes:
    let _my_char = '🚀';
}