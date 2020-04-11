use std::fmt;

//pub type Result<T> = std::result::Result<T, Error>;

// XXX: mb separate errors by modules for eventual lib splitting
#[derive(Debug)]
pub enum Error {
    Hook,
    // options parse errors
    ParseStatusKeys,
    ParseVersion,
    ParseWindowFlag,
    ParseSwitch,
    ParseSetClipboard,
    ParseActivity,
    ParseAction,
    ParseStatus,
    ParseWindowSize,
    ParseStatusJustify,
    ParseStatusPosition,
    ParseClockModeStyle,
    ParsePaneBorderStatus,

    /// Tmux error message
    Tmux(String),
    /// IO error
    IO(std::io::Error),

    ParseInt(std::num::ParseIntError),
    Parse(std::string::ParseError),
}

//impl Error {
//pub fn new(error: &str) -> Self {
//}
//}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Self::IO(ref err) => Some(err),
            Self::ParseInt(ref err) => Some(err),
            Self::Parse(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Tmux(ref msg) => write!(f, "{}", msg),
            Self::IO(ref err) => err.fmt(f),
            Self::ParseInt(ref err) => err.fmt(f),
            Self::Parse(ref err) => err.fmt(f),
            _ => "".fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<std::string::ParseError> for Error {
    fn from(err: std::string::ParseError) -> Self {
        Self::Parse(err)
    }
}
