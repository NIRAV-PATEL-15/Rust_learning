use std::collections::HashMap;

fn main() {
    /// Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    
    /// Strings 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    
    // Concatenate strings using the `+` operator
    let s3 = s1 + &s2; 
    println!("{s3}");

    /// Hashmap
    let mut scores = HashMap::new();

    // Insert key-value pairs into the hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Note: This will overwrite the previous value for the key "Blue"

    // Print the hashmap
    println!("{:?}", scores);

    // Using entry to insert a value only if the key does not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    
    // Using entry to insert a value only if the key does not exist (this time for "Blue")
    scores.entry(String::from("Blue")).or_insert(50);

    // Print the updated hashmap
    println!("{:?}", scores);
}
