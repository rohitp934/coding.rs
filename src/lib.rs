#![allow(clippy::needless_return)]

use actix_web::{HttpResponse, Responder};
use log::{debug, error};
use regex::Regex;
use std::{
    path::Path,
    time::{Duration, Instant},
};
use tokio::{io::AsyncReadExt, process::Command};
use types::{CodingError, ErrorResponse, Question};
use uuid::Uuid;
// Library to spawn process in parallel and execute
// source code in various languages.
// Author: @rohitp934
// License: MIT
// Version: 0.1.0
pub mod types;

struct Program {
    file_name: String,
    binary_name: Option<String>,
    language: String,
    time_limit: i32,
    is_compiled: bool,
}

struct CompiledProgram {
    src_file_path: String,
    language: String,
    file_name_without_ext: String,
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
        }
        "c" => Ok(String::from("main.c")),
        "cpp" => Ok(String::from("main.cpp")),
        "csharp" => Ok(String::from("main.cs")),
        "go" => Ok(String::from("main.go")),
        "javascript" => Ok(String::from("index.js")),
        "julia" => Ok(String::from("main.jl")),
        "kotlin" => Ok(String::from("main.kt")),
        "python" => Ok(String::from("main.py")),
        "ruby" => Ok(String::from("main.rb")),
        "rust" => Ok(String::from("main.rs")),
        "scala" => {
            let re = Regex::new(r"object\s+(\w+)\s*\{").unwrap();
            if let Some(captures) = re.captures(src) {
                let class_name = captures.get(1).unwrap().as_str();
                Ok(format!("{}.scala", class_name))
            } else {
                Err(CodingError::InvalidPublicClass)
            }
        }
        "swift" => Ok(String::from("main.swift")),
        "typescript" => Ok(String::from("index.ts")),
        "zig" => Ok(String::from("main.zig")),
        _ => Ok(String::from("")),
    }
}

fn get_file_name_without_ext(file_name: &str) -> Result<&str, CodingError> {
    let file_stem = Path::new(file_name).file_stem();
    match file_stem {
        Some(no_ext) => {
            let no_ext = no_ext.to_str().unwrap();
            Ok(no_ext)
        }
        None => Err(CodingError::FileNameError),
    }
}

fn is_compiled_language(language: &str) -> bool {
    let compiled_langs = vec!["java", "cpp", "c", "rust", "csharp"];
    if compiled_langs.contains(&language) {
        return true;
    }
    false
}

async fn init(question: &Question) -> Result<(String, String), CodingError> {
    let file_name = make_filename(&question.language, &question.source_code)?;
    let folder_name = format!("{}{}", question.language, Uuid::new_v4());
    if let Err(err) = tokio::fs::create_dir_all(format!("tmp/{}", folder_name)).await {
        error!(
            "Something went wrong when trying to create the subdirectories :: {}",
            err.to_string()
        );
        return Err(CodingError::FileCreationError);
    };
    let src_file_path = format!("tmp/{}/{}", folder_name, file_name);
    if let Err(err) = tokio::fs::write(&src_file_path, &question.source_code).await {
        error!(
            "Something went wrong when trying to create the source file. {}",
            err.to_string()
        );
        return Err(CodingError::FileCreationError);
    }
    debug!("Source file created successfully.");
    Ok((file_name, folder_name))
}

impl CompiledProgram {
    async fn compile(&self) -> Result<(), CodingError> {
        // Check if files are present
        if !Path::new(&self.src_file_path).exists() {
            return Err(CodingError::FileError);
        }

        // Check if language is supported
        let cmd: String;
        let args: String;

        match self.language.as_str() {
            "java" => {
                cmd = String::from("javac");
                args = self.src_file_path.to_string();
            }
            "c" => {
                cmd = String::from("gcc");
                args = format!("{} -o {}", self.src_file_path, self.file_name_without_ext);
            }
            "cpp" => {
                cmd = String::from("g++");
                args = format!("{} -o {}", self.src_file_path, self.file_name_without_ext);
            }
            "c#" => {
                cmd = String::from("mcs");
                args = self.src_file_path.to_string();
            }
            "kotlin" => {
                cmd = String::from("kotlinc");
                args = format!(
                    "{} -include-runtime -d {}.jar",
                    self.src_file_path, self.file_name_without_ext
                );
            }
            "rust" => {
                cmd = String::from("rustc");
                args = format!("{} -o {}", self.src_file_path, self.file_name_without_ext);
            }
            "scala" => {
                cmd = String::from("scalac");
                args = self.src_file_path.to_string();
            }
            "swift" => {
                cmd = String::from("swiftc");
                args = self.src_file_path.to_string();
            }
            "typescript" => {
                cmd = String::from("npx tsc");
                args = self.src_file_path.to_string();
            }
            "zig" => {
                cmd = String::from("zig");
                args = format!("build-exe {}", self.src_file_path);
            }
            _ => {
                return Err(CodingError::FileError);
            }
        }

        let child = Command::new(&cmd)
            .args(args.split_whitespace())
            .output()
            .await;
        match child {
            Err(_) => Err(CodingError::ProcessError),
            Ok(output) => {
                // Get the stderr output as a &str
                if let Ok(stderr) = String::from_utf8(output.stderr) {
                    // Check for errors
                    debug!("{}", stderr);
                    if output.status.code() != Some(0) {
                        Err(CodingError::CompileError)
                    } else {
                        Ok(())
                    }
                } else {
                    Err(CodingError::InvalidStringFromConsole)
                }
            }
        }
    }
}

impl Program {
    async fn run(&self) -> Result<(), CodingError> {
        // Create the run command based on the language
        let cmd: String;
        let args: String;

        if self.is_compiled {
            let binary = self.binary_name.as_ref().unwrap();

            // Create the run command based on the language
            // specifically for compiled languages
            match self.language.as_str() {
                "java" => {
                    cmd = String::from("java");
                    args = format!("{}.class", binary);
                }
                "c" => {
                    cmd = format!("./{}", binary);
                    args = String::new();
                }
                "cpp" => {
                    cmd = format!("./{}", binary);
                    args = String::new();
                }
                "c#" => {
                    cmd = String::from("mono");
                    args = format!("{}.exe", binary);
                }
                "kotlin" => {
                    cmd = String::from("java");
                    args = format!("-jar {}.jar", binary);
                }
                "rust" => {
                    cmd = format!("./{}", binary);
                    args = String::new();
                }
                "scala" => {
                    cmd = String::from("scala");
                    args = format!("{}.class", binary);
                }
                "swift" => {
                    cmd = String::from("swift");
                    args = binary.to_string();
                }
                "typescript" => {
                    cmd = String::from("node");
                    args = format!("{}.js", binary);
                }
                "zig" => {
                    cmd = format!("./{}", binary);
                    args = String::new();
                }
                _ => {
                    return Err(CodingError::FileError);
                }
            }
            let mut stdout = String::new();
            let mut stderr = String::new();

            let mut run_binary = Command::new(&cmd)
                .args(args.split_whitespace())
                .spawn()
                .unwrap();
            // Set a time limit
            let timeout = Duration::from_secs(self.time_limit as u64);
            let start = Instant::now();
            loop {
                if start.elapsed() > timeout {
                    // println!("Time limit exceeded for: {}", self.folder_name);
                    return Err(CodingError::TimeLimitExceeded);
                }
                match run_binary.try_wait() {
                    Ok(None) => {
                        // println!("Program still running for: {}", self.folder_name);
                        continue;
                    }
                    Err(_e) => {
                        // println!("Program failed for: {}", self.folder_name);
                        run_binary
                            .stderr
                            .unwrap()
                            .read_to_string(&mut stderr)
                            .await
                            .unwrap();
                        return Err(CodingError::InvalidStringFromConsole);
                    }
                    Ok(Some(status)) => {
                        if status.success() {
                            run_binary
                                .stdout
                                .unwrap()
                                .read_to_string(&mut stdout)
                                .await
                                .unwrap();
                            //? Need to return the stdout
                            return Ok(());
                        } else {
                            run_binary
                                .stderr
                                .unwrap()
                                .read_to_string(&mut stderr)
                                .await
                                .unwrap();
                            return Err(CodingError::InvalidStringFromConsole);
                        }
                    }
                }
            }
        } else {
            // Create the run command based on the language
            // specifically for interpreted languages
            match self.language.as_str() {
                "python" => {
                    cmd = String::from("python3");
                    args = self.file_name.to_string();
                }
                "ruby" => {
                    cmd = String::from("ruby");
                    args = self.file_name.to_string();
                }
                "javascript" => {
                    cmd = String::from("node");
                    args = self.file_name.to_string();
                }
                "go" => {
                    cmd = String::from("go");
                    args = format!("run {}", self.file_name);
                }
                "julia" => {
                    cmd = String::from("julia");
                    args = self.file_name.to_string();
                }
                _ => {
                    return Err(CodingError::FileError);
                }
            }
            let run_binary = Command::new(&cmd)
                .args(args.split_whitespace())
                .output()
                .await;
            match run_binary {
                Err(_) => return Err(CodingError::ProcessError),
                Ok(output) => {
                    // Get the stderr output as a &str
                    if let Ok(stderr) = String::from_utf8(output.stderr) {
                        // Check for errors
                        debug!("{}", stderr);
                        if output.status.code() != Some(0) {
                            return Err(CodingError::CompileError);
                        } else {
                            return Ok(());
                        }
                    } else {
                        return Err(CodingError::InvalidStringFromConsole);
                    }
                }
            }
        }
    }
}

async fn cleanup(folder_name: &str) -> Result<(), CodingError> {
    match tokio::fs::remove_dir_all(&folder_name).await {
        Err(err) => {
            error!("Cleanup Error :: {}", err.to_string());
            Err(CodingError::CleanupError)
        }
        Ok(()) => Ok(()),
    }
}

pub async fn execute(question: Question) -> impl Responder {
    let src = match init(&question).await {
        Ok(init_response) => init_response,
        Err(err) => {
            error!("Something went wrong for id: {}!\n{}", &question.id, err);
            return HttpResponse::BadRequest().json(ErrorResponse {
                id: question.id,
                error: err.to_string(),
            });
        }
    };
    let (file_name, folder_name) = src;
    let src_file_path = format!("tmp/{}/{}", folder_name, file_name);
    debug!("Source file path: {}", src_file_path);
    let file_name_no_ext = match get_file_name_without_ext(&file_name) {
        Ok(res) => res,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                id: question.id,
                error: err.to_string(),
            });
        }
    };
    let mut is_compiled = false;
    if is_compiled_language(&question.language) {
        is_compiled = true;
        let compilation_program = CompiledProgram {
            src_file_path: src_file_path.clone(),
            file_name_without_ext: format!("tmp/{}/{}", folder_name, file_name_no_ext),
            language: question.language.clone(),
        };
        let compile_output = compilation_program.compile().await;
        if let Err(err) = compile_output {
            error!("Compilation Error for id: {}! :: {}", &question.id, err);
            if let Err(err) = cleanup(&format!("tmp/{}", folder_name)).await {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    id: question.id,
                    error: err.to_string(),
                });
            }
            let response = ErrorResponse {
                id: question.id,
                error: err.to_string(),
            };
            return HttpResponse::BadRequest().json(response);
        }
    }
    let executed_program = Program {
        file_name: src_file_path.clone(),
        binary_name: if is_compiled {
            Some(file_name_no_ext.to_string())
        } else {
            None
        },
        language: question.language.clone(),
        time_limit: question.timeout,
        is_compiled,
    };
    let _run_output = executed_program.run().await;

    if let Err(err) = cleanup(&format!("tmp/{}", folder_name)).await {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            id: question.id,
            error: err.to_string(),
        });
    }
    HttpResponse::Ok().json(String::from("Compilation successful"))
}
