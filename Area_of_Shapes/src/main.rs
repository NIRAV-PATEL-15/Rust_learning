use std::io;

/// A struct representing a Rectangle
struct Rectangle {
    /// Height of the rectangle
    height: f64,
    /// Width of the rectangle
    width: f64,
}

/// Implementation of methods to calculate properties of a rectangle
impl Rectangle {
    /// Calculates the area of the rectangle
    fn area_of_rectangle(&self) -> f64 {
        self.height * self.width
    }

    /// Calculates the area of a square (if rectangle is a square)
    fn area_of_square(&self) -> f64 {
        if self.height < self.width {
            self.height * self.height
        } else {
            self.width * self.width
        }
    }

    /// Calculates the area of a circle (assuming the rectangle can inscribe a circle)
    fn area_of_circle(&self) -> f64 {
        const PI: f64 = 3.142;
        let radius = if self.height < self.width {
            self.height / 2.0
        } else {
            self.width / 2.0
        };
        PI * radius * radius
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter height and width separated by space");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");

    let numbers: Vec<f64> = input
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter valid numbers"))
        .collect();

    let rectangle1 = Rectangle {
        height: numbers[0],
        width: numbers[1],
    };

    println!("Area of rectangle : {}", rectangle1.area_of_rectangle());
    println!("Area of square : {}", rectangle1.area_of_square());
    println!("Area of circle : {}", rectangle1.area_of_circle());
}
