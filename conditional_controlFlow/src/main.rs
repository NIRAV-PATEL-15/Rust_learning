// temp convert 
use std::io;
fn main() {
   println!("Enter input type (f/c) :");
   let mut temp_type = String::new();
   io::stdin().read_line(&mut temp_type).expect("invalid input");
   println!("enter value :");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("unble to read");
   let  temp_value : f32 = input.trim().parse().expect("invalid input");

   if temp_type.trim() == "f" {
      let cel_value: f32 = (temp_value - 32.0) * (5.0 / 9.0);
      println!("{} is {} degree Celsius.", temp_value, cel_value);
  } else if temp_type.trim() == "c" {
      let far_value = (temp_value * (9.0 / 5.0)) + 32.0;
      println!("{} is {} degree Fahrenheit.", temp_value, far_value.round());
  } else {
      println!("Invalid type");
  }
}
