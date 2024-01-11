use crate::basic_ops::dmas::TwoDigit; // Import the TwoDigit struct from the basic_ops module
use std::io;

// Import the basic_ops module (assuming it's defined in a separate file named basic_ops.rs)
pub mod basic_ops;

fn main() {
    let mut input = String::new();

    // Prompt the user to enter two numbers separated by space
    println!("Enter two numbers separated by space");

    // Read the user input into the input variable
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input string into a vector of i32 numbers
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter valid numbers"))
        .collect();

    // Check if exactly two numbers are entered
    if numbers.len() != 2 {
        println!("Please enter exactly two numbers separated by space.");
        return;
    }

    // Create a TwoDigit instance with the entered numbers
    let points = TwoDigit(numbers[0], numbers[1]);

    // Print the sum of the entered numbers using the add method of the TwoDigit struct
    println!("Sum of entered numbers is: {}", points.add());
}
