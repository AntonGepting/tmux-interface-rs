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
    TwoRows,
    #[cfg(feature = "tmux_2_9")]
    ThreeRows,
    #[cfg(feature = "tmux_2_9")]
    FourRows,
    #[cfg(feature = "tmux_2_9")]
    FiveRows,
}

#[cfg(feature = "tmux_0_8")]
impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ON => Ok(Self::On),
            OFF => Ok(Self::Off),
            #[cfg(feature = "tmux_2_9")]
            NUMBER_2 => Ok(Self::TwoRows),
            #[cfg(feature = "tmux_2_9")]
            NUMBER_3 => Ok(Self::ThreeRows),
            #[cfg(feature = "tmux_2_9")]
            NUMBER_4 => Ok(Self::FourRows),
            #[cfg(feature = "tmux_2_9")]
            NUMBER_5 => Ok(Self::FiveRows),
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
            Self::TwoRows => write!(f, "{}", NUMBER_2),
            #[cfg(feature = "tmux_2_9")]
            Self::ThreeRows => write!(f, "{}", NUMBER_3),
            #[cfg(feature = "tmux_2_9")]
            Self::FourRows => write!(f, "{}", NUMBER_4),
            #[cfg(feature = "tmux_2_9")]
            Self::FiveRows => write!(f, "{}", NUMBER_5),
        }
    }
}
