use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//clock-mode-style [12 | 24]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_1_0")]
pub enum ClockModeStyle {
    H12,
    H24,
}

#[cfg(feature = "tmux_1_0")]
impl FromStr for ClockModeStyle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            H12 => Ok(Self::H12),
            H24 => Ok(Self::H24),
            _ => Err(Error::ParseClockModeStyle),
        }
    }
}

#[cfg(feature = "tmux_1_0")]
impl fmt::Display for ClockModeStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::H12 => write!(f, "{}", H12),
            Self::H24 => write!(f, "{}", H24),
        }
    }
}
