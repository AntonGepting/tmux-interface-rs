use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

// destroy-unattached [off | on | keep-last | keep-group]
#[cfg(feature = "tmux_1_5")]
#[derive(PartialEq, Clone, Debug)]
pub enum DestroyUnattached {
    Off,
    On,
    #[cfg(feature = "tmux_3_4")]
    KeepLast,
    #[cfg(feature = "tmux_3_4")]
    KeepGroup,
}

#[cfg(feature = "tmux_1_5")]
impl fmt::Display for DestroyUnattached {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Off => write!(f, "{}", OFF),
            Self::On => write!(f, "{}", ON),
            #[cfg(feature = "tmux_3_4")]
            Self::KeepLast => write!(f, "{}", KEEP_LAST),
            #[cfg(feature = "tmux_3_4")]
            Self::KeepGroup => write!(f, "{}", KEEP_GROUP),
        }
    }
}

#[cfg(feature = "tmux_1_5")]
impl FromStr for DestroyUnattached {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            OFF => Ok(Self::Off),
            ON => Ok(Self::On),
            #[cfg(feature = "tmux_3_4")]
            KEEP_LAST => Ok(Self::KeepLast),
            #[cfg(feature = "tmux_3_4")]
            KEEP_GROUP => Ok(Self::KeepGroup),
            _ => Err(Error::ParseAction),
        }
    }
}
