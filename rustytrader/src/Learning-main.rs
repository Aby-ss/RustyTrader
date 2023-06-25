use std::io;

fn main() {
    // Prompt the user for the first number
    println!("Enter the first number:");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read input.");
    let first_number: i32 = first_number
        .trim()
        .parse()
        .expect("Invalid input. Please enter a number.");

    // Prompt the user for the second number
    println!("Enter the second number:");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read input.");
    let second_number: i32 = second_number
        .trim()
        .parse()
        .expect("Invalid input. Please enter a number.");

    // Add the numbers
    let sum = first_number + second_number;

    // Print the result
    println!("The sum of {} and {} is: {}", first_number, second_number, sum);
}
