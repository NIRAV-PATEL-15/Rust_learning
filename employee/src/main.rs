use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct EmployeeInfo {
    name: String,
    age: Option<u8>,
    skills: Vec<String>,
    position: Option<String>,
    #[serde(rename(serialize = "experiance(y)", deserialize = "experiance(y)"))]
    experience_in_years: Option<u8>,
}

fn main() {
    let employee_file = fs::read_to_string("./src/Employee.json");
    match employee_file {
        Ok(employee_file) => {
            let data: Result<Vec<EmployeeInfo>, serde_json::Error> =
                serde_json::from_str(&employee_file);

            match data {
                Ok(employee_data) => {
                    let mut mid_rust = Vec::new();
                    let mut jr_java = Vec::new();
                    let mut sr_or_c = Vec::new();
                    
                    for i in 0..employee_data.len() {
                        let emp_data = EmployeeInfo {
                            name: employee_data[i].name.clone(),
                            age: employee_data[i].age,
                            skills: employee_data[i].skills.clone(),
                            position: employee_data[i].position.clone(),
                            experience_in_years: employee_data[i].experience_in_years,
                        };
                        if employee_data[i].position == Some("Software Developer".to_string())
                            && employee_data[i].skills.contains(&"Rust".to_string())
                        {
                            mid_rust.push(emp_data);
                        } else if employee_data[i].position
                            == Some("Jr. Software Developer".to_string())
                            && employee_data[i].skills.contains(&"Java".to_string())
                        {
                            jr_java.push(emp_data)
                        } else if employee_data[i].position
                            == Some("Sr. Software Developer".to_string())
                            || employee_data[i].skills.contains(&"C#".to_string())
                        {
                            sr_or_c.push(emp_data)
                        }
                    }

                    let mid_rust_data =
                        serde_json::to_string_pretty(&mid_rust).expect("Failed to convert");
                    let jr_java_data =
                        serde_json::to_string_pretty(&jr_java).expect("Failed to convert");
                    let sr_or_c_data =
                        serde_json::to_string_pretty(&sr_or_c).expect("Failed to convert");
                    fs::write("./src/data/MidRust_Dev.json", &mid_rust_data)
                        .expect("Failed to write file");
                    fs::write("./src/data/JrJava_Dev.json", &jr_java_data)
                        .expect("Failed to write file");
                    fs::write("./src/data/Sr_C#_Dev.json", &sr_or_c_data)
                        .expect("Failed to write file");
                }
                Err(_) => {}
            }
        }
        Err(err) => {
            println!("Error occured , failed to read file.");
        }
    }
}
