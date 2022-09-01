use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//status-position [top | bottom]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_1_7")]
pub enum StatusPosition {
    Top,
    Bottom,
}

#[cfg(feature = "tmux_1_7")]
impl FromStr for StatusPosition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            TOP => Ok(Self::Top),
            BOTTOM => Ok(Self::Bottom),
            _ => Err(Error::ParseStatusPosition),
        }
    }
}

#[cfg(feature = "tmux_1_7")]
impl fmt::Display for StatusPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Top => write!(f, "{}", TOP),
            Self::Bottom => write!(f, "{}", BOTTOM),
        }
    }
}
