use crate::basic_ops::dmas::TwoDigit;
use std::io;
pub mod basic_ops;
fn main() {
    let mut input = String::new();
    println!("Enter two numbers separated by space");
    io::stdin().read_line(&mut input).expect("failed to readline");
    let numbers :Vec<i32> = input.split_whitespace().map(|num| num.parse().expect("Please enter valid numbers")).collect();
    if numbers.len() != 2 {
        println!("Please enter exactly two numbers separated by space.");
        return;
    }
    let points = TwoDigit(numbers[0],numbers[1]);
    println!("sum of entered number is : {}",points.add())
}
