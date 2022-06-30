use std::fmt;
use std::str::FromStr;

const ON: &str = "on";
const FOCUS: &str = "focus";
const LEFT_MARKER: &str = "left-marker";
const RIGHT_MARKER: &str = "right-marker";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum List {
    On,
    Focus,
    LeftMarker,
    RightMarker,
}

impl fmt::Display for List {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::On => ON,
            Self::Focus => FOCUS,
            Self::LeftMarker => LEFT_MARKER,
            Self::RightMarker => RIGHT_MARKER,
        };
        write!(f, "{}", s)
    }
}

impl FromStr for List {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ON => Ok(Self::On),
            FOCUS => Ok(Self::Focus),
            LEFT_MARKER => Ok(Self::LeftMarker),
            RIGHT_MARKER => Ok(Self::RightMarker),
            _ => Err(()),
        }
    }
}
