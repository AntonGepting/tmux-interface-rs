// ```
// tmux >=3.2:
// ```
// used by commands:
//  * refresh-client

use std::fmt;
use std::str::FromStr;

const STATE_ON: &str = "on";
const STATE_OFF: &str = "off";
const STATE_CONTINUE: &str = "continue";
const STATE_PAUSE: &str = "pause";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum State {
    On,
    Off,
    Continue,
    Pause,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::On => STATE_ON,
            Self::Off => STATE_OFF,
            Self::Continue => STATE_CONTINUE,
            Self::Pause => STATE_PAUSE,
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ParseStateError;

impl<'a> FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            STATE_ON => Ok(Self::On),
            STATE_OFF => Ok(Self::Off),
            STATE_CONTINUE => Ok(Self::Continue),
            STATE_PAUSE => Ok(Self::Pause),
            _ => Err(ParseStateError),
        }
    }
}
