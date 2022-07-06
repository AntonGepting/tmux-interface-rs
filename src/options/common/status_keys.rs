use super::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
pub enum StatusKeys {
    Vi,
    Emacs,
}

impl fmt::Display for StatusKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Vi => write!(f, "{}", VI),
            Self::Emacs => write!(f, "{}", EMACS),
        }
    }
}

impl FromStr for StatusKeys {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            VI => Ok(Self::Vi),
            EMACS => Ok(Self::Emacs),
            _ => Err(Error::ParseStatusKeys),
        }
    }
}
