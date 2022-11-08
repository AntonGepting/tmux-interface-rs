use crate::options::common::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//set-clipboard [on | external | off]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_3_2")]
pub enum ExtendedKeys {
    On,
    Off,
    #[cfg(feature = "tmux_3_2a")]
    Always,
}

#[cfg(feature = "tmux_3_2")]
impl fmt::Display for ExtendedKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "{}", ON),
            Self::Off => write!(f, "{}", OFF),
            #[cfg(feature = "tmux_3_2a")]
            Self::Always => write!(f, "{}", ALWAYS),
        }
    }
}

#[cfg(feature = "tmux_3_2")]
impl FromStr for ExtendedKeys {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            #[cfg(feature = "tmux_3_2a")]
            ALWAYS => Ok(Self::Always),
            _ => Err(Error::ParseExtendedKeys),
        }
    }
}
