
use crate::log;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    CustomError(String),
}

impl std::error::Error for MyError {}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::CustomError(msg) => write!(f, "CustomError: {}", msg),
        }
    }
}


pub fn diagnostic_fatal() -> Result<(), MyError> {
    let result: Result<(), MyError> = Err(MyError::CustomError("[inspect] [diagnostic_fatal]".to_string()));
    result
}

pub fn diagnostic_panic() -> Result<(), MyError> {
    let result: Result<(), MyError> = Err(MyError::CustomError("[inspect] [diagnostic_panic]".to_string()));
    result
}

pub fn diagnostic_info(msg:&str) {
    log::log::logger("info", "diagnostic_info", msg);
}