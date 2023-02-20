use actix_web::{Responder, HttpResponse};
use log::{error, info};
use regex::Regex;
use types::{Question, CodingError, ErrorResponse};
use uuid::Uuid;

// Library to spawn process in parallel and execute
// source code in various languages.
// Author: @rohitp934
// License: MIT
// Version: 0.1.0
pub mod types;

struct Program {
    file_name: String,
    folder_name: String,
    name: String,
    language: String,
    index: i32,
    input_file: String,
    expected_output_file: String,
    actual_output_file: String,
    time_limit: i32,
}

struct CompiledProgram {
    
}

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

fn is_compiled_language(language: &str) -> bool {
    let compiled_langs = vec!["java", "cpp", "c", "rust", "csharp"];
    if compiled_langs.contains(&language) {
        return true;
    }
    false
}

async fn init(question: &Question) -> Result<String, CodingError> {
    let file_name = make_filename(&question.language, &question.source_code)?;
    info!("ID: {}, The file name is {}", question.id, file_name);
    let folder_name = format!("{}{}", question.language, Uuid::new_v4());
    let src_file_path = format!("tmp/{}/{}", folder_name, file_name);
    if tokio::fs::write(&src_file_path, &question.source_code).await.is_err() {
        return Err(CodingError::FileCreationError);
    }
    Ok(String::from(src_file_path))
}

async fn compile(question: &Question, src_file) -> Result<(), CodingError> {
    // Remove previous executables
    if Path::new(&self.name).exists() {
        fs::remove_file(&self.name).unwrap();
    }
    // Create a new variable with string.
    // Check if files are present
    if !Path::new(&self.file_name).exists() {
        return (StatusCodes::FileNotFound, String::from("Missing file"));
    }

    // Check if language is supported
    let cmd: String;
    let args: String;

    match self.language.as_str() {
        "java" => {
            cmd = String::from("javac");
            args = self.file_name.to_string();
        }
        "c" => {
            cmd = String::from("gcc");
            args = format!("-o {} {}", self.file_name, self.name);
        }
        "cpp" => {
            cmd = String::from("g++");
            args = format!("-o {} {}", self.file_name, self.name);
        }
        "rust" => {
            cmd = String::from("rustc");
            args = format!("-o {} {}", self.file_name, self.name);
        }
        "c#" => {
            cmd = String::from("mcs");
            args = self.file_name.to_string();
        }
        _ => {
            return (
                StatusCodes::InvalidFile,
                String::from("Unsupported language"),
            );
        }
    }

    let output = Command::new(&cmd)
        .args(args.split_whitespace())
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute command: {}", e);
        });

    // Get the stderr output as a &str
    let stderr = String::from_utf8(output.stderr).expect("Found invalid UTF-8");

    // Check for errors
    if output.status.code() != Some(0) {
        (StatusCodes::CompilationError, stderr)
    } else {
        // println!("Compilation successful for: {}", self.folder_name);
        (StatusCodes::Ok, String::from("Success"))
    }
}

pub async fn execute(question: Question) -> impl Responder {
    let src_file_path = init(&question).await;
    if let Err(err) = src_file_path {
        error!("Something went wrong for id: {}!\n{}", &question.id, err);
        let response = ErrorResponse {
            id: question.id,
            error: err.to_string()
        };
        return HttpResponse::BadRequest().json(response);
    }
    if is_compiled_language(&question.language) {
        let response = compile(&question, &src_file_path).await;
    }
    
    HttpResponse::Ok().json(String::from("Hello World"))
}
