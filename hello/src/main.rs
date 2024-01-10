// fn main() {
//    // let name = "Nirav";
//    let mut x = 5;
//    println!("sum of 2 num is {}",sum(x,4));
//    x = 20;
//    println!("mul of 2 num is {}",mul(x,4))
// }

// fn sum(x:i32,y:i32)->i32{
//    x+y
// }

// fn mul(x:i32,y:i32)->i32{
//    x*y
// }


//Guessing Game 
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
println!("Guess the number!! ");
let secret_num = rand::thread_rng().gen_range(1..=100);
println!("Input the number :");
let mut guess = String::new();
io::stdin().read_line(&mut guess)
.expect("failed to read line");
// println!("Your secret number is  : {secret_num}");
println!("you have guessed : {guess}");

let guess :u32 = guess.trim().parse().expect("please type number");
match  guess.cmp(&secret_num) {
   Ordering::Less => println!("Too small!"),
   Ordering::Greater => println!("Too big!"),
   Ordering::Equal => println!("You win!"),
    
}
}