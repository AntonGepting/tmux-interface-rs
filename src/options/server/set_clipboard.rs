use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//set-clipboard [on | external | off]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_1_5")]
pub enum SetClipboard {
    On,
    Off,
    #[cfg(feature = "tmux_2_6")]
    External,
}

#[cfg(feature = "tmux_1_5")]
impl fmt::Display for SetClipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::On => ON,
            Self::Off => OFF,
            #[cfg(feature = "tmux_2_6")]
            Self::External => EXTERNAL,
        };
        f.write_str(value)
    }
}

#[cfg(feature = "tmux_1_5")]
impl FromStr for SetClipboard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            #[cfg(feature = "tmux_2_6")]
            EXTERNAL => Ok(Self::External),
            _ => Err(Error::ParseSetClipboard),
        }
    }
}
