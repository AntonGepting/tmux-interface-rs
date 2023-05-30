use crate::options::common::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

// mode-mouse [on | off | copy-mode]
#[derive(PartialEq, Clone, Debug)]
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub enum ModeMouse {
    On,
    Off,
    CopyMode,
}

#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub const COPY_MODE: &str = "copy-mode";

#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
impl FromStr for ModeMouse {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            COPY_MODE => Ok(Self::CopyMode),
            _ => Err(Error::ParseModeMouse),
        }
    }
}

#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
impl fmt::Display for ModeMouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "{}", ON),
            Self::Off => write!(f, "{}", OFF),
            Self::CopyMode => write!(f, "{}", COPY_MODE),
        }
    }
}
