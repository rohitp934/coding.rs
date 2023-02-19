use log::{error, info};
use regex::Regex;
use types::{Question, CodingError};

// Library to spawn process in parallel and execute
// source code in various languages.
// Author: @rohitp934
// License: MIT
// Version: 0.1.0
pub mod types;

fn make_filename(language: &str, src: &str) -> Result<String, CodingError> {
    match language {
        "java" => {
            let re = Regex::new(r"public\s+class\s+(\w+)\s*\{").unwrap();
            if let Some(captures) = re.captures(src) {
                let class_name = captures.get(1).unwrap().as_str();
                println!("Public class name: {}", class_name);
                Ok(String::from(format!("{}.java", class_name)))
            } else {
                Err(CodingError::InvalidPublicClass)
            }
        },
        _ => {
            Ok(String::from(""))
        }
    }
}

pub fn code_checker(question: Question) -> String {
    
    match make_filename(&question.language, &question.source_code) {
        Ok(file_name) => {
            info!("The file name is {}", file_name);
        }
        Err(error) => {
            error!("Something went wrong for id: {}!\n{}", &question.id, error);
        }
    }
    String::from("Hello World")
}
