use std::fmt;
use std::io;
use std::error::Error;


/// Project_cfg error
#[derive(Debug)]
pub struct TmuxInterfaceError {
    /// The formatted error message
    pub err_text: String,
    /// The type of error
    pub err_type: usize
}

impl TmuxInterfaceError {
    pub fn new(error: &str) -> Self {
        TmuxInterfaceError {
            err_text: error.to_string(),
            err_type: 0
        }
    }
}


impl Error for TmuxInterfaceError {
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
impl From<io::Error> for TmuxInterfaceError {
    fn from(_error: io::Error) -> Self {
        TmuxInterfaceError {
            err_text: String::from("io"),
            err_type: 1
            //message: error.to_string(),
        }
    }
}


//
impl From<regex::Error> for TmuxInterfaceError {
    fn from(_error: regex::Error) -> Self {
        TmuxInterfaceError {
            err_text: String::from("regex"),
            err_type: 1
            //message: error.to_string(),
        }
    }
}


impl From<std::num::ParseIntError> for TmuxInterfaceError {
    fn from(_error: std::num::ParseIntError) -> Self {
        TmuxInterfaceError {
            err_text: String::from("parse"),
            err_type: 1
            //message: error.to_string(),
        }
    }
}


impl From<std::string::ParseError> for TmuxInterfaceError {
    fn from(_error: std::string::ParseError) -> Self {
        TmuxInterfaceError {
            err_text: String::from("parse"),
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


impl fmt::Display for TmuxInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err_text)
    }
}


