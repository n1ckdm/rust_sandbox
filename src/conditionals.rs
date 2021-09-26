pub fn run() {
    let age = 18;
    let check = false;
    let knows = true;

    if check && age >= 18 || knows {
        println!("You're 18 or older");
    } else if check && age < 18 {
        println!("You're younger than 18");
    } else {
        println!("No check required")
    }

    // No turnary operator with, but can do shorthand if:
    let is_of_age = if age >= 18 { true } else { false };
    println!("{:?}", is_of_age);
}