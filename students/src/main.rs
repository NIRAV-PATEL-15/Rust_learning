use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

/// Represents a Student structure with various fields.
#[derive(Debug, Deserialize, Serialize)]
struct Student {
    name: String,
    email: String,
    marks: Vec<u16>,
    percentage: f32,
    grade: char,
    address: String,
    city: String,
    phone: String,
}

/// Calculates the percentage based on a vector of u32 marks.
fn calculate_percentage(marks: &Vec<u16>) -> f32 {
    let total: u16 = marks.iter().sum();
    (total as f32) / (marks.len() as f32)
}

/// Calculates the grade based on the percentage.
fn calculate_grade(percentage: f32) -> char {
    match percentage {
        p if p >= 90.0 => 'A',
        p if p >= 80.0 => 'B',
        p if p >= 70.0 => 'C',
        p if p >= 60.0 => 'D',
        _ => 'F',
    }
}

fn main() {
    let path = "./updated_student_data.json";
    /// Read the content of the file using fs module
    let file_data = fs::read_to_string(path).expect("failed to read file");

    /// Convert JSON data into string
    let data: Vec<serde_json::Value> =serde_json::from_str(&file_data).expect("failed to parse json");

    /// Iterate through each object in the vector
    let new_data: Vec<Student> = data
        .iter()
        .map(|student| {
            /// Deserialize the JSON value representing student marks
            let marks: Vec<u16> =
                serde_json::from_value(student["marks"].clone()).expect("failed to get value");

            /// Calculate percentage and grade
            let percentage = calculate_percentage(&marks);
            let grade = calculate_grade(percentage);

            /// Map or transform data into Student Struct
            Student {
                name: serde_json::from_value(student["name"].clone()).expect("failed to get value"),
                email: serde_json::from_value(student["email"].clone())
                    .expect("failed to get value"),
                address: serde_json::from_value(student["address"].clone())
                    .expect("failed to get value"),
                city: serde_json::from_value(student["city"].clone()).expect("failed to get value"),
                phone: serde_json::from_value(student["phone"].clone())
                    .expect("failed to get value"),
                marks,
                percentage,
                grade,
        }
        })
        .collect(); // Collect is used to gather all the values and return them as a vector

    /// Serialize 'new_data' into a formatted JSON string
    let updated_json = serde_json::to_string_pretty(&new_data).expect("Failed to serialize data");

    /// Write updated data into a new JSON file using fs module
    fs::write("updated_student_data.json", updated_json).expect("failed to write file");
}
