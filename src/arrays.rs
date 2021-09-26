// Arrays are fixed list where elements all share the same type

// Bring in a library:
use std::mem::size_of_val;

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    // Get single value:
    println!("index 2: {}", numbers[2]);

    // Memory allocation:
    println!("Array size: {} bytes", size_of_val(&numbers));

    // Get a slice:
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}