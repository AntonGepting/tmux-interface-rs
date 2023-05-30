use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "tmux_1_4")]
#[derive(PartialEq, Clone, Debug)]
pub enum DetachOnDestroy {
    On,
    Off,
    #[cfg(feature = "tmux_3_2")]
    NoDetached,
}

#[cfg(feature = "tmux_1_4")]
impl FromStr for DetachOnDestroy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            #[cfg(feature = "tmux_3_2")]
            NO_DETACHED => Ok(Self::NoDetached),
            _ => Err(Error::ParseDetachOnDestroy),
        }
    }
}

#[cfg(feature = "tmux_1_4")]
impl fmt::Display for DetachOnDestroy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "{}", ON),
            Self::Off => write!(f, "{}", OFF),
            #[cfg(feature = "tmux_3_2")]
            Self::NoDetached => write!(f, "{}", NO_DETACHED),
        }
    }
}
