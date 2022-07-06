use crate::Error;
use std::fmt;
use std::str::FromStr;

//window-size largest | smallest | manual | latest
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_2_9")]
pub enum WindowSize {
    Largest,
    Smallest,
    Manual,
    #[cfg(feature = "tmux_3_1")]
    Latest,
}

#[cfg(feature = "tmux_2_9")]
impl FromStr for WindowSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            LARGEST => Ok(Self::Largest),
            SMALLEST => Ok(Self::Smallest),
            MANUAL => Ok(Self::Manual),
            #[cfg(feature = "tmux_3_1")]
            LATEST => Ok(Self::Latest),
            _ => Err(Error::ParseWindowSize),
        }
    }
}

#[cfg(feature = "tmux_2_9")]
impl fmt::Display for WindowSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Largest => write!(f, "{}", LARGEST),
            Self::Smallest => write!(f, "{}", SMALLEST),
            Self::Manual => write!(f, "{}", MANUAL),
            #[cfg(feature = "tmux_3_1")]
            Self::Latest => write!(f, "{}", LATEST),
        }
    }
}
