// Temperature conversion program
use std::io;

fn main() {
    // Prompt the user to enter the input type (fahrenheit or celsius)
    println!("Enter input type (f/c):");

    // Read the user input into the temp_type variable
    let mut temp_type = String::new();
    io::stdin().read_line(&mut temp_type).expect("Invalid input");

    // Prompt the user to enter the temperature value
    println!("Enter value:");

    // Read the user input into the input variable
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read");

    // Parse the input string into a floating-point number
    let temp_value: f32 = input.trim().parse().expect("Invalid input");

    // Check the input type and perform the corresponding temperature conversion
    if temp_type.trim() == "f" {
        // Convert Fahrenheit to Celsius
        let cel_value: f32 = (temp_value - 32.0) * (5.0 / 9.0);
        println!("{} is {} degree Celsius.", temp_value, cel_value);
    } else if temp_type.trim() == "c" {
        // Convert Celsius to Fahrenheit
        let far_value = (temp_value * (9.0 / 5.0)) + 32.0;
        println!("{} is {} degree Fahrenheit.", temp_value, far_value.round());
    } else {
        // Invalid input type
        println!("Invalid type");
    }
}
