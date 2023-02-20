use serde::{Deserialize, Serialize};
use snafu::Snafu;

#[derive(Deserialize)]
pub struct Question {
    pub id: String,
    pub language: String,
    pub source_code: String,
    pub timeout: i32,
    pub sample_testcases: Vec<String>,
}

#[derive(Debug, Snafu)]
pub enum CodingError {
    #[snafu(display("The given Java source code does not have a valid public class.\nExpected something like: `public class Main`."))]
    InvalidPublicClass,
    #[snafu(display("Unable to create file."))]
    FileCreationError,
    #[snafu(display("Something went wrong during File I/O op."))]
    FileError,
    #[snafu(display("Something went wrong during execution of child process."))]
    ProcessError,
    #[snafu(display("Invalid utf8 character found in std console of child process."))]
    InvalidStringFromConsole,
    #[snafu(display("An error occurred during compilation of source code."))]
    CompileError,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub id: String,
    pub error: String,
}
