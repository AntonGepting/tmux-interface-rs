use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//status-justify [left | centre | right]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_1_0")]
pub enum StatusJustify {
    Left,
    Centre,
    Right,
}

#[cfg(feature = "tmux_1_0")]
impl FromStr for StatusJustify {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            LEFT => Ok(Self::Left),
            CENTRE => Ok(Self::Centre),
            RIGHT => Ok(Self::Right),
            _ => Err(Error::ParseStatusJustify),
        }
    }
}

#[cfg(feature = "tmux_1_0")]
impl fmt::Display for StatusJustify {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => write!(f, "{}", LEFT),
            Self::Centre => write!(f, "{}", CENTRE),
            Self::Right => write!(f, "{}", RIGHT),
        }
    }
}
