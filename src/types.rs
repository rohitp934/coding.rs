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
}


#[derive(Serialize)]
pub struct ErrorResponse {
    pub id: String,
    pub error: String
}