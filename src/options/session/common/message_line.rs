use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "tmux_3_4")]
#[derive(PartialEq, Clone, Debug)]
pub enum MessageLine {
    Ln0,
    Ln1,
    Ln2,
    Ln3,
    Ln4,
}

#[cfg(feature = "tmux_3_4")]
impl fmt::Display for MessageLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ln0 => write!(f, "{}", NUMBER_0),
            Self::Ln1 => write!(f, "{}", NUMBER_1),
            Self::Ln2 => write!(f, "{}", NUMBER_2),
            Self::Ln3 => write!(f, "{}", NUMBER_3),
            Self::Ln4 => write!(f, "{}", NUMBER_4),
        }
    }
}

#[cfg(feature = "tmux_3_4")]
impl FromStr for MessageLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            NUMBER_0 => Ok(Self::Ln0),
            NUMBER_1 => Ok(Self::Ln1),
            NUMBER_2 => Ok(Self::Ln2),
            NUMBER_3 => Ok(Self::Ln3),
            NUMBER_4 => Ok(Self::Ln4),
            _ => Err(Error::ParseAction),
        }
    }
}
