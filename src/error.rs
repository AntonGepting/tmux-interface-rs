use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    A,
    B,
    ParseVersionError,
}

/// Project_cfg error
#[derive(Debug)]
pub struct Error {
    /// The formatted error message
    pub message: String,
    /// The type of error
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(error: &str) -> Self {
        Error {
            message: error.to_string(),
            kind: ErrorKind::A,
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "invalid first item to double"
    }
    //fn cause(&self) -> Option<&Error> {
    //match self.error_type {
    //_ => None,
    //}
    //}
}

impl From<std::io::Error> for Error {
    fn from(_error: std::io::Error) -> Self {
        Error {
            message: String::from("io"),
            kind: ErrorKind::A, //message: error.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_error: std::num::ParseIntError) -> Self {
        Error {
            message: String::from("parse num"),
            kind: ErrorKind::A, //message: error.to_string(),
        }
    }
}

impl From<std::string::ParseError> for Error {
    fn from(_error: std::string::ParseError) -> Self {
        Error {
            message: String::from("parse string"),
            kind: ErrorKind::A, //message: error.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
