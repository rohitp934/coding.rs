use actix_web::{Responder, HttpResponse};
use log::{error, info};
use regex::Regex;
use types::{Question, CodingError, ErrorResponse};

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
                Ok(format!("{}.java", class_name))
            } else {
                Err(CodingError::InvalidPublicClass)
            }
        },
        _ => {
            Ok(String::from(""))
        }
    }
}

fn init(question: &Question) -> Result<(), CodingError> {
    let file_name = make_filename(&question.language, &question.source_code)?;
    info!("ID: {}, The file name is {}", question.id, file_name);
    Ok(())
}

pub fn execute(question: Question) -> impl Responder {
    if let Err(err) = init(&question) {
        error!("Something went wrong for id: {}!\n{}", &question.id, err);
        let response = ErrorResponse {
            id: question.id,
            error: err.to_string()
        };
        return HttpResponse::BadRequest().json(response);
    }
    
    HttpResponse::Ok().json(String::from("Hello World"))
}
