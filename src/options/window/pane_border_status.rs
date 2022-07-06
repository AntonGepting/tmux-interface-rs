use crate::options::common::constants::*;
use crate::Error;
use std::fmt;
use std::str::FromStr;

//pane-border-status [off | top | bottom]
#[derive(PartialEq, Clone, Debug)]
#[cfg(feature = "tmux_2_3")]
pub enum PaneBorderStatus {
    Off,
    Top,
    Bottom,
}

#[cfg(feature = "tmux_2_3")]
impl FromStr for PaneBorderStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            OFF => Ok(Self::Off),
            TOP => Ok(Self::Top),
            BOTTOM => Ok(Self::Bottom),
            _ => Err(Error::ParsePaneBorderStatus),
        }
    }
}

#[cfg(feature = "tmux_2_3")]
impl fmt::Display for PaneBorderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Off => write!(f, "{}", OFF),
            Self::Top => write!(f, "{}", TOP),
            Self::Bottom => write!(f, "{}", BOTTOM),
        }
    }
}
