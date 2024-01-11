use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Attempt to open the file "hello.txt"
    let greeting_file_result = File::open("hello.txt");

    // Use a match statement to handle the Result from File::open
    let greeting_file = match greeting_file_result {
        Ok(file) => file, // If the file is successfully opened, store it in greeting_file
        Err(error) => match error.kind() {
            // If the error is of kind NotFound, try creating the file
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc, // If the file is successfully created, store it in greeting_file
                Err(e) => panic!("Problem creating the file: {:?}", e), // Panic if there is an error creating the file
            },
            // If the error is not NotFound, panic with the original error
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
