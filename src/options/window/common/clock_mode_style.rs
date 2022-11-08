use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//clock-mode-style [12 | 24]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_1_0")]
pub enum ClockModeStyle {
    _12,
    _24,
}

#[cfg(feature = "tmux_1_0")]
impl FromStr for ClockModeStyle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            _12 => Ok(Self::_12),
            _24 => Ok(Self::_24),
            _ => Err(Error::ParseClockModeStyle),
        }
    }
}

#[cfg(feature = "tmux_1_0")]
impl fmt::Display for ClockModeStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::_12 => write!(f, "{}", _12),
            Self::_24 => write!(f, "{}", _24),
        }
    }
}
