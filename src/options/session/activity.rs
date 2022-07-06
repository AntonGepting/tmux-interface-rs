use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//visual-silence [on | off | both]
#[cfg(feature = "tmux_0_8")]
#[derive(PartialEq, Clone, Debug)]
pub enum Activity {
    On,
    Off,
    #[cfg(feature = "tmux_2_6")]
    Both,
}

#[cfg(feature = "tmux_0_8")]
impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "{}", ON),
            Self::Off => write!(f, "{}", OFF),
            #[cfg(feature = "tmux_2_6")]
            Self::Both => write!(f, "{}", BOTH),
        }
    }
}

#[cfg(feature = "tmux_0_8")]
impl FromStr for Activity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            #[cfg(feature = "tmux_2_6")]
            BOTH => Ok(Self::Both),
            _ => Err(Error::ParseActivity),
        }
    }
}
