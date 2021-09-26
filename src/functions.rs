pub fn run() {
    // Standard functions
    greeting("hi", "Nick");
    println!("{:?}", add(5,5));

    // Closures (lamdas)
    let z = 5;
    let add_nums = |x: i32, y: i32| x + y + z;
    println!("{:?}", add_nums(5,5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}