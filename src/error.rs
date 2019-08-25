use std::fmt;
//use std::io;
//use std::error;


/// Project_cfg error
#[derive(Debug)]
pub struct Error {
    /// The formatted error message
    pub err_text: String,
    /// The type of error
    pub err_type: usize
}


impl Error {
    pub fn new(error: &str) -> Self {
        Error {
            err_text: error.to_string(),
            err_type: 0
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


// Implement std::convert::From for MyError; from io::Error
impl From<std::io::Error> for Error {
    fn from(_error: std::io::Error) -> Self {
        Error {
            err_text: String::from("io"),
            err_type: 1
            //message: error.to_string(),
        }
    }
}


impl From<std::num::ParseIntError> for Error {
    fn from(_error: std::num::ParseIntError) -> Self {
        Error {
            err_text: String::from("parse num"),
            err_type: 1
            //message: error.to_string(),
        }
    }
}


impl From<std::string::ParseError> for Error {
    fn from(_error: std::string::ParseError) -> Self {
        Error {
            err_text: String::from("parse string"),
            err_type: 1
            //message: error.to_string(),
        }
    }
}


//impl From<std::option::NoneError> for TmuxInterfaceError {
    //fn from(_error: std::option::NoneError) -> Self {
        //TmuxInterfaceError {
            //err_text: String::from("parse"),
            //err_type: 1
            ////message: error.to_string(),
        //}
    //}
//}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err_text)
    }
}


