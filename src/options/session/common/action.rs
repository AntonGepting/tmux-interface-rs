use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//activity-action [any | none | current | other]
//bell-action [any | none | current | other]
//silence-action [any | none | current | other]
#[cfg(feature = "tmux_0_8")]
#[derive(PartialEq, Clone, Debug)]
pub enum Action {
    Any,
    None,
    Current,
    #[cfg(feature = "tmux_2_1")]
    Other,
}

#[cfg(feature = "tmux_0_8")]
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Any => write!(f, "{}", ANY),
            Self::None => write!(f, "{}", NONE),
            Self::Current => write!(f, "{}", CURRENT),
            #[cfg(feature = "tmux_2_1")]
            Self::Other => write!(f, "{}", OTHER),
        }
    }
}

#[cfg(feature = "tmux_0_8")]
impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            ANY => Ok(Self::Any),
            NONE => Ok(Self::None),
            CURRENT => Ok(Self::Current),
            #[cfg(feature = "tmux_2_1")]
            OTHER => Ok(Self::Other),
            _ => Err(Error::ParseAction),
        }
    }
}
