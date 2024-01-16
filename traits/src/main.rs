use std::f32::consts::PI;
// defining trait for area
trait Area {
    fn calculate_area(&self)->f32;
}

//struct for circle
struct Circle {radius:f32}
//struct for square
struct Square {length:f32}
//struct for rectangle
struct Rectangle {height:f32,width:f32}
//defining behaviour for Circle
impl Area for Circle{
    fn calculate_area(&self)->f32{
        PI * self.radius * self.radius 
    }
}
//defining behaviour for Rectangle
impl Area for Rectangle{
    fn calculate_area(&self)->f32{
        self.height * self.width
    }
}
//idefining behaviour for Square
impl Area for Square{
    fn calculate_area(&self)->f32{
      self.length * self.length
    }
}

fn main() {
   let circle = Circle{radius:4.5};
   let square = Square{length:5.0};
   let rectangle = Rectangle{height:4.5,width:5.5};
   println!("Area of Circle is {}",circle.calculate_area());
   println!("Area of Square is {}",square.calculate_area());
   println!("Area of Rectangle is {}",rectangle.calculate_area());
}
