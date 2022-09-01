use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//status [off | on | 2 | 3 | 4 | 5]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_0_8")]
pub enum Status {
    On,
    Off,
    #[cfg(feature = "tmux_2_9")]
    _2,
    #[cfg(feature = "tmux_2_9")]
    _3,
    #[cfg(feature = "tmux_2_9")]
    _4,
    #[cfg(feature = "tmux_2_9")]
    _5,
}

#[cfg(feature = "tmux_0_8")]
impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            #[cfg(feature = "tmux_2_9")]
            _2 => Ok(Self::_2),
            #[cfg(feature = "tmux_2_9")]
            _3 => Ok(Self::_3),
            #[cfg(feature = "tmux_2_9")]
            _4 => Ok(Self::_4),
            #[cfg(feature = "tmux_2_9")]
            _5 => Ok(Self::_5),
            _ => Err(Error::ParseStatus),
        }
    }
}

#[cfg(feature = "tmux_0_8")]
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "{}", ON),
            Self::Off => write!(f, "{}", OFF),
            #[cfg(feature = "tmux_2_9")]
            Self::_2 => write!(f, "{}", _2),
            #[cfg(feature = "tmux_2_9")]
            Self::_3 => write!(f, "{}", _3),
            #[cfg(feature = "tmux_2_9")]
            Self::_4 => write!(f, "{}", _4),
            #[cfg(feature = "tmux_2_9")]
            Self::_5 => write!(f, "{}", _5),
        }
    }
}
