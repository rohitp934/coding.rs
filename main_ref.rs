use std::{
    env,
    fs::{self, read_to_string, File, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use serde_json::{json, to_string_pretty};

#[repr(i32)]
#[derive(PartialEq)]
enum StatusCodes {
    Ok = 200,
    Accepted = 201,
    WrongAnswer = 400,
    CompilationError = 401,
    RuntimeError = 402,
    InvalidFile = 403,
    FileNotFound = 404,
    TimeLimitExceeded = 408,
    InternalServerError = 500,
}

impl StatusCodes {
    fn message(&self) -> &str {
        match *self {
            StatusCodes::Ok => "Success",
            StatusCodes::Accepted => "Accepted",
            StatusCodes::WrongAnswer => "Wrong Answer",
            StatusCodes::CompilationError => "Compilation Error",
            StatusCodes::RuntimeError => "Runtime Error",
            StatusCodes::InvalidFile => "Invalid File",
            StatusCodes::FileNotFound => "File Not Found",
            StatusCodes::TimeLimitExceeded => "Time Limit Exceeded",
            StatusCodes::InternalServerError => "Internal Server Error",
        }
    }
}

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

impl Program {
    #[allow(clippy::too_many_arguments)]
    fn new(
        filename: &str,
        index: i32,
        language: &str,
        inputfile: &str,
        timelimit: i32,
        expectedoutputfile: &str,
        is_sample_exec: &str,
        is_custom_input: bool,
    ) -> Program {
        let folder_name = get_parent_folder_name(filename);
        if is_sample_exec == "submit" {
            Program {
                file_name: filename.to_string(),
                folder_name: folder_name.to_string(),
                language: language.to_string(),
                index,
                name: if language == "java" {
                    get_java_file_stem(filename)
                } else {
                    format!("{}/Program", folder_name)
                },
                input_file: inputfile.to_string(),
                expected_output_file: expectedoutputfile.to_string(),
                actual_output_file: format!("{}/actualoutput{}.txt", folder_name, index),
                time_limit: timelimit,
            }
        } else if is_custom_input {
            Program {
                file_name: filename.to_string(),
                folder_name: folder_name.to_string(),
                language: language.to_string(),
                index,
                name: if language == "java" {
                    get_java_file_stem(filename)
                } else {
                    format!("{}/Program", folder_name)
                },
                input_file: inputfile.to_string(),
                expected_output_file: String::new(),
                actual_output_file: format!("{}/actualoutput{}.txt", folder_name, index),
                time_limit: timelimit,
            }
        } else {
            Program {
                file_name: filename.to_string(),
                folder_name: folder_name.to_string(),
                language: language.to_string(),
                index,
                name: if language == "java" {
                    get_java_file_stem(filename)
                } else {
                    format!("{}/Program", folder_name)
                },
                input_file: inputfile.to_string(),
                expected_output_file: expectedoutputfile.to_string(),
                actual_output_file: String::new(),
                time_limit: timelimit,
            }
        }
    }

    fn compile(&self) -> (StatusCodes, String) {
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

    fn run_custom_input(&self) -> (StatusCodes, String) {
        // Check if files are present
        if !Path::new(&self.file_name).exists() {
            return (
                StatusCodes::FileNotFound,
                String::from("Missing executable file"),
            );
        }

        // Check if input file is present
        if !Path::new(&self.input_file).exists() {
            return (
                StatusCodes::FileNotFound,
                String::from("Missing input file"),
            );
        }

        // Check if language is supported
        let cmd: String;
        let args: String;

        match self.language.as_str() {
            "java" => {
                cmd = String::from("java");
                args = self.name.to_string();
            }
            "c" | "cpp" | "rust" => {
                cmd = String::from("./");
                args = self.name.to_string();
            }
            "c#" => {
                cmd = String::from("mono");
                args = format!("{}.exe", self.name);
            }
            "ruby" => {
                cmd = String::from("ruby");
                args = self.file_name.to_string();
            }
            "python" => {
                cmd = String::from("python3");
                args = self.file_name.to_string();
            }
            "go" => {
                cmd = String::from("go");
                args = format!("run {}", self.name);
            }
            "javascript" => {
                cmd = String::from("node");
                args = format!("--harmony {}", self.file_name);
            }
            _ => {
                return (
                    StatusCodes::InvalidFile,
                    String::from("Unsupported language"),
                );
            }
        }

        let mut output = match self.language.as_str() {
            "java" => Command::new("cd")
                .arg(&self.folder_name)
                .spawn()
                .and_then(|_| Command::new(&cmd).args(args.split_whitespace()).spawn())
                .unwrap_or_else(|e| {
                    panic!("Failed to execute command: {}", e);
                }),
            _ => Command::new(&cmd)
                .args(args.split_whitespace())
                .spawn()
                .unwrap_or_else(|e| {
                    panic!("Failed to execute command: {}", e);
                }),
        };

        let mut stdout = String::new();
        let mut stderr = String::new();

        // Set a time limit
        let timeout = Duration::from_secs(self.time_limit as u64);
        let start = Instant::now();

        loop {
            if start.elapsed() > timeout {
                // println!("Time limit exceeded for: {}", self.folder_name);
                return (
                    StatusCodes::TimeLimitExceeded,
                    String::from("Time limit exceeded"),
                );
            }
            match output.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        // println!("Program ran successfully for: {}", self.folder_name);
                        output.stdout.unwrap().read_to_string(&mut stdout).unwrap();
                        return (StatusCodes::Accepted, stdout);
                    } else {
                        // println!("Program failed for: {}", self.folder_name);
                        output.stderr.unwrap().read_to_string(&mut stderr).unwrap();
                        return (StatusCodes::RuntimeError, stderr);
                    }
                }
                Ok(None) => {
                    // println!("Program still running for: {}", self.folder_name);
                    continue;
                }
                Err(_e) => {
                    // println!("Program failed for: {}", self.folder_name);
                    output.stderr.unwrap().read_to_string(&mut stderr).unwrap();
                    return (StatusCodes::InternalServerError, stderr);
                }
            }
        }
    }

    fn run_sample(&self) -> (StatusCodes, String) {
        // Check if files are present
        if !Path::new(&self.file_name).exists() {
            return (
                StatusCodes::FileNotFound,
                String::from("Missing executable file"),
            );
        }

        // Check if language is supported
        let cmd: String;
        let args: String;

        match self.language.as_str() {
            "java" => {
                cmd = String::from("java");
                args = self.name.to_string();
            }
            "c" | "cpp" | "rust" => {
                cmd = String::from("./");
                args = self.name.to_string();
            }
            "c#" => {
                cmd = String::from("mono");
                args = format!("{}.exe", self.name);
            }
            "ruby" => {
                cmd = String::from("ruby");
                args = self.file_name.to_string();
            }
            "python" => {
                cmd = String::from("python3");
                args = self.file_name.to_string();
            }
            "go" => {
                cmd = String::from("go");
                args = format!("run {}", self.name);
            }
            "javascript" => {
                cmd = String::from("node");
                args = format!("--harmony {}", self.file_name);
            }
            _ => {
                return (
                    StatusCodes::InvalidFile,
                    String::from("Unsupported language"),
                );
            }
        }

        let mut output = match self.language.as_str() {
            "java" => Command::new("cd")
                .arg(&self.folder_name)
                .spawn()
                .and_then(|_| Command::new(&cmd).args(args.split_whitespace()).spawn())
                .unwrap_or_else(|e| {
                    panic!("Failed to execute command: {}", e);
                }),
            _ => Command::new(&cmd)
                .args(args.split_whitespace())
                .spawn()
                .unwrap_or_else(|e| {
                    panic!("Failed to execute command: {}", e);
                }),
        };

        let mut stdout = String::new();
        let mut stderr = String::new();

        // Set a time limit
        let timeout = Duration::from_secs(self.time_limit as u64);
        let start = Instant::now();

        loop {
            if start.elapsed() > timeout {
                // println!("Time limit exceeded for: {}", self.folder_name);
                return (
                    StatusCodes::TimeLimitExceeded,
                    String::from("Time limit exceeded"),
                );
            }
            match output.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        // println!("Program ran successfully for: {}", self.folder_name);
                        output.stdout.unwrap().read_to_string(&mut stdout).unwrap();
                        return (StatusCodes::Accepted, stdout);
                    } else {
                        // println!("Program failed for: {}", self.folder_name);
                        output.stderr.unwrap().read_to_string(&mut stderr).unwrap();
                        return (StatusCodes::RuntimeError, stderr);
                    }
                }
                Ok(None) => {
                    // println!("Program still running for: {}", self.folder_name);
                    continue;
                }
                Err(_e) => {
                    // println!("Program failed for: {}", self.folder_name);
                    output.stderr.unwrap().read_to_string(&mut stderr).unwrap();
                    return (StatusCodes::InternalServerError, stderr);
                }
            }
        }
    }

    fn run(&self) -> (StatusCodes, String) {
        // Check if files are present
        let curr_dir = env::current_dir().unwrap();
        let curr_dir = curr_dir.display();
        if !Path::new(&format!("{}/{}", &curr_dir, &self.file_name)).exists() {
            return (
                StatusCodes::FileNotFound,
                String::from("Missing executable file"),
            );
        }

        // Check if language is supported
        let cmd: String;
        let args: String;

        match self.language.as_str() {
            "java" => {
                cmd = String::from("java");
                args = self.name.to_string();
            }
            "c" | "cpp" | "rust" => {
                cmd = String::from("./");
                args = self.name.to_string();
            }
            "c#" => {
                cmd = String::from("mono");
                args = format!("{}.exe", self.name);
            }
            "ruby" => {
                cmd = String::from("ruby");
                args = self.file_name.to_string();
            }
            "python" => {
                cmd = String::from("python");
                args = format!("{}/{}", &curr_dir, self.file_name);
            }
            "go" => {
                cmd = String::from("go");
                args = format!("run {}", self.name);
            }
            "javascript" => {
                cmd = String::from("node");
                args = format!("--harmony {}", self.file_name);
            }
            _ => {
                return (
                    StatusCodes::InvalidFile,
                    String::from("Unsupported language"),
                );
            }
        }

        let fout = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.actual_output_file)
            .unwrap();
        let stderr_file = format!("{}/stderr{}.txt", self.folder_name, self.index);
        let ferr = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&stderr_file)
            .unwrap();

        let fin = File::open(&self.input_file).unwrap();

        let args = args.split_whitespace();
        let mut output = match self.language.as_str() {
            "java" => Command::new("cd")
                .arg(&self.folder_name)
                .spawn()
                .and_then(|_| {
                    Command::new(&cmd)
                        .args(args)
                        .stdin(Stdio::from(fin))
                        .stdout(Stdio::from(fout))
                        .stderr(Stdio::from(ferr))
                        .spawn()
                })
                .unwrap_or_else(|e| {
                    panic!("Failed to execute command: {}", e);
                }),
            _ => Command::new(&cmd)
                .args(args)
                .stdin(Stdio::from(fin))
                .stdout(Stdio::from(fout))
                .stderr(Stdio::from(ferr))
                .spawn()
                .unwrap_or_else(|e| {
                    panic!("Failed to execute command: {}", e);
                }),
        };

        // Set a time limit
        let timeout = Duration::from_secs(self.time_limit as u64);
        let start = Instant::now();

        loop {
            if start.elapsed() > timeout {
                // println!("Time limit exceeded for: {}", self.folder_name);
                return (
                    StatusCodes::TimeLimitExceeded,
                    String::from("Time limit exceeded"),
                );
            }

            match output.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        return self.match_output();
                    } else {
                        // Read stderr file
                        let err = read_to_string(&stderr_file).unwrap();
                        return (StatusCodes::RuntimeError, format!("Runtime error: {}", err));
                    }
                }
                Ok(None) => {
                    // println!("Program still running for: {}", self.folder_name);
                    continue;
                }
                Err(_e) => {
                    // println!("Program failed for: {}", self.folder_name);
                    return (
                        StatusCodes::InternalServerError,
                        String::from("Internal server error"),
                    );
                }
            }
        }
    }

    fn match_output(&self) -> (StatusCodes, String) {
        if !Path::new(&self.actual_output_file).exists()
            || !Path::new(&self.expected_output_file).exists()
        {
            return (
                StatusCodes::FileNotFound,
                String::from("Missing output file"),
            );
        }
        let mut contents = read_to_string(&self.actual_output_file).unwrap();
        contents = contents.trim().to_string();
        let mut fout = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.actual_output_file)
            .unwrap();
        // fout.seek(SeekFrom::Start(0)).unwrap();
        fout.write_all(contents.as_bytes()).unwrap();

        let result = filecmp::cmp(&self.actual_output_file, &self.expected_output_file, false);
        match result {
            Ok(true) => {
                // println!("Program passed for: {}", self.folder_name);
                (StatusCodes::Accepted, String::from("Success"))
            }
            Ok(false) => {
                // println!("Program failed for: {}", self.folder_name);
                (StatusCodes::WrongAnswer, String::from("Wrong answer"))
            }
            Err(_e) => {
                // println!("Program failed for: {}", self.folder_name);
                (
                    StatusCodes::InternalServerError,
                    String::from("Internal server error"),
                )
            }
        }
    }
}

fn get_parent_folder_name(filename: &str) -> &str {
    if let Some(folder_name) = Path::new(filename)
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|f| f.to_str())
    // .map(|s| s.to_string())
    {
        folder_name
    } else {
        panic!("Unable to get parent folder name");
    }
}

fn get_java_file_stem(filename: &str) -> String {
    if let Some(file_stem) = Path::new(filename)
        .file_stem()
        .and_then(|f| f.to_str())
        .map(|s| s.to_string())
    {
        file_stem
    } else {
        panic!("Unable to get file stem");
    }
}

#[allow(clippy::too_many_arguments)]
fn codechecker(
    language: &str,
    is_sample_exec: &String,
    filename: &str,
    compile: bool,
    index: i32,
    inputfile: Option<&String>,
    expectedoutput: Option<&String>,
    timeout: i32,
    is_custom_input: bool,
) {
    // Verify that if is_sample_exec is submit, then inputfile and expectedoutput are not None
    if is_sample_exec == "submit" && (inputfile.is_none() || expectedoutput.is_none()) {
        panic!("Input file and expected output file are required for submission");
    }

    // Now extract the input file and expected output file from the options
    // If they are none, then use empty strings
    let default_file_name = String::from("");
    let inputfile = inputfile.unwrap_or(&default_file_name);
    let expectedoutput = expectedoutput.unwrap_or(&default_file_name);
    let new_program = Program::new(
        filename,
        index,
        language,
        inputfile,
        timeout,
        expectedoutput,
        is_sample_exec,
        is_custom_input,
    );

    if compile {
        let (status, errors) = new_program.compile();
        if status as i32 == 401 {
            // println!("Compilation failed for: {}", new_program.folder_name);
            // println!("{}", output);
            let response = json!({
                "status": status as i32,
                "error": Some(errors),
            });
            let json_string = to_string_pretty(&response).unwrap();
            println!("{}", json_string);
            return;
        }
        let response = json!({
            "status": status as i32,
        });
        let json_string = to_string_pretty(&response).unwrap();
        println!("{}", json_string);
        // Exit the program
        return;
    }

    let runtime_result: StatusCodes;
    let console_output: String;
    if is_sample_exec == "run" {
        if is_custom_input {
            (runtime_result, console_output) = new_program.run_custom_input()
        } else {
            (runtime_result, console_output) = new_program.run_sample()
        }
    } else {
        (runtime_result, console_output) = new_program.run()
    }

    let status_code = runtime_result as i32;
    let mut json_string = json!({ "status": status_code });
    let response = json_string.as_object_mut().unwrap();

    if !console_output.is_empty() {
        if runtime_result == StatusCodes::RuntimeError {
            response.insert("error".to_string(), json!(console_output));
        } else {
            response.insert("error".to_string(), json!(runtime_result.message()));
        }

        if status_code >= 400 {
            let json_string = to_string_pretty(&response).unwrap();
            println!("{}", json_string);
            return;
        }
    }

    // If the program is a sample program, then we don't need to check for output
    // And we return the console output as a part of the response
    if is_sample_exec == "run" {
        response.insert("debugOutput".to_string(), json!(console_output));
    }

    // Match expected output if is_sample_exec is submit
    if is_sample_exec == "submit" {
        let (match_result, match_errors) = new_program.match_output();
        response.insert("status".to_string(), json!(match_result as i32));
        if !match_errors.is_empty() {
            response.insert("error".to_string(), json!(match_errors));
        }
        let json_string = to_string_pretty(&response).unwrap();
        println!("{}", json_string);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let language = args.get(1).unwrap();
    if language == "sql" {
        let _is_sample_exec = args.get(2).unwrap();
        // sqlchecker(
        //     filename: &args[3],
        //     expectedoutput: &args[4],
        //     timeout: &args[5],
        //     sqlconf: &args[6],
        //     is_sample_exec: is_sample_exec,
        // );
    } else {
        let is_sample_exec = args.get(2).unwrap();
        let compile = args.get(4).unwrap();
        if compile.to_lowercase() == "true" {
            codechecker(
                language,
                is_sample_exec,
                args.get(3).unwrap(),
                true,
                -1,
                None,
                None,
                1,
                false,
            );
        } else if is_sample_exec == "run" {
            let custom_testcase = args.get(5).unwrap();
            if custom_testcase.to_lowercase() == "true" {
                let timeout: i32 = args.get(7).unwrap().parse().unwrap();
                codechecker(
                    language,
                    is_sample_exec,
                    args.get(3).unwrap(),
                    false,
                    -1,
                    args.get(6),
                    None,
                    timeout,
                    true,
                );
            } else {
                let timeout: i32 = args.get(6).unwrap().parse().unwrap();
                codechecker(
                    language,
                    is_sample_exec,
                    args.get(3).unwrap(),
                    false,
                    -1,
                    None,
                    None,
                    timeout,
                    false,
                );
            }
        } else {
            let timeout: i32 = args.get(8).unwrap().parse().unwrap();
            let index: i32 = args.get(5).unwrap().parse().unwrap();
            let inputfile = args.get(6);
            let expectedoutput = args.get(7);
            codechecker(
                language,
                is_sample_exec,
                args.get(3).unwrap(),
                false,
                index,
                inputfile,
                expectedoutput,
                timeout,
                false,
            )
        }
    }
}
