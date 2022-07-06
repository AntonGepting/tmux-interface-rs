use super::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
pub enum Switch {
    On,
    Off,
}

impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "{}", ON),
            Self::Off => write!(f, "{}", OFF),
        }
    }
}

impl FromStr for Switch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            _ => Err(Error::ParseSwitch),
        }
    }
}
