use std::fmt;
use std::str::FromStr;

const LEFT: &str = "left";
const CENTRE: &str = "centre";
const RIGHT: &str = "right";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Align {
    Left,
    Centre,
    Right,
}

impl fmt::Display for Align {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Left => LEFT,
            Self::Centre => CENTRE,
            Self::Right => RIGHT,
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Align {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            LEFT => Ok(Self::Left),
            CENTRE => Ok(Self::Centre),
            RIGHT => Ok(Self::Right),
            _ => Err(()),
        }
    }
}
