// Vectors are resizable arrays

// Bring in a library:
use std::mem::size_of_val;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Add to the end
    numbers.push(6);
    numbers.push(7);

    //Pop off the last value:
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value:
    println!("index 2: {}", numbers[2]);

    // Memory allocation:
    println!("Vector size: {} bytes", size_of_val(&numbers));

    // Get a slice:
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through a vector:
    for x in numbers.iter() {
        print!("{:?}", x);
    }
    println!("Hi");

    // Loop through a vector:
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}