use super::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_3_0")]
pub enum RemainOnExit {
    On,
    Off,
    #[cfg(feature = "tmux_3_2")]
    Failed,
}

#[cfg(feature = "tmux_3_0")]
impl fmt::Display for RemainOnExit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "{}", ON),
            Self::Off => write!(f, "{}", OFF),
            #[cfg(feature = "tmux_3_2")]
            Self::Failed => write!(f, "{}", FAILED),
        }
    }
}

#[cfg(feature = "tmux_3_0")]
impl FromStr for RemainOnExit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            #[cfg(feature = "tmux_3_2")]
            FAILED => Ok(Self::Failed),
            _ => Err(Error::ParseRemainOnExit),
        }
    }
}
