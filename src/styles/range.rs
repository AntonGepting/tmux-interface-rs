use std::fmt;

const LEFT: &str = "left";
const RIGHT: &str = "right";
// WINDOW

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Range {
    Left,
    Right,
    Window(u32),
}

impl fmt::Display for Range {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Left => LEFT.to_string(),
            Self::Right => RIGHT.to_string(),
            Self::Window(index) => format!("{}|{}", "window", index),
        };
        write!(f, "{}", s)
    }
}

//use std::str::FromStr;

//// TODO: window
//impl FromStr for Range {
//type Err = ();

//fn from_str(s: &str) -> Result<Self, Self::Err> {
//unimplemented!();
//match s {
//LEFT => Ok(Self::Left),
//RIGHT => Ok(Self::Right),
//_ => Err(()),
//}
//}
//}
