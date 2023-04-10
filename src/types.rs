use serde::{Deserialize, Serialize};
use snafu::Snafu;

#[derive(Deserialize)]
pub struct Question {
    pub id: String,
    pub language: String,
    pub source_code: String,
    pub timeout: i32,
    pub sample_testcases: Vec<(String, String)>,
}

#[derive(Debug, Snafu)]
pub enum CodingError {
    #[snafu(display("InvalidPublicClass :: The given Java source code does not have a valid public class.\nExpected something like: `public class Main`."))]
    InvalidPublicClass,
    #[snafu(display("FileCreationError :: Unable to create file."))]
    FileCreationError,
    #[snafu(display("FileError :: Unable to extract file stem."))]
    FileNameError,
    #[snafu(display("FileError :: Something went wrong during File I/O op."))]
    FileError,
    #[snafu(display("ProcessError :: Something went wrong during execution of child process."))]
    ProcessError,
    #[snafu(display(
        "InvalidStringFromConsole :: Invalid utf8 character found in std console of child process."
    ))]
    InvalidStringFromConsole,
    #[snafu(display("CompileError :: An error occurred during compilation of source code."))]
    CompileError,
    #[snafu(display("CleanupError :: An error occurred during cleanup of source code."))]
    CleanupError,
    #[snafu(display("TimeLimitExceeded :: The user's program exceeded the time limit."))]
    TimeLimitExceeded,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub id: String,
    pub error: String,
}
