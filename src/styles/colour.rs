use std::borrow::Cow;
use std::fmt;

const BLACK: &str = "black";
const RED: &str = "red";
const GREEN: &str = "green";
const YELLOW: &str = "yellow";
const BLUE: &str = "blue";
const MAGENTA: &str = "magenta";
const CYAN: &str = "cyan";
const WHITE: &str = "white";
const BRIGHTRED: &str = "brightred";
const BRIGHTGREEN: &str = "brightgreen";
const BRIGHTYELLOW: &str = "brightyellow";
// COLOURSET
const DEFAULT: &str = "default";
const TERMINAL: &str = "terminal";
// HEX

/// if supported the bright variants brightred, brightgreen, brightyellow
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Colour {
    /// black
    Black,
    /// red
    Red,
    /// green
    Green,
    /// yellow
    Yellow,
    /// blue
    Blue,
    /// magenta
    Magenta,
    /// cyan
    Cyan,
    /// white
    White,
    /// brightred
    BrightRed,
    /// brightgreen
    BrightGreen,
    /// brightyellow
    BrightYellow,
    /// colour0 to colour255 from the 256-colour set
    ColourSet256(u8),
    /// default colour
    Default,
    /// terminal default colour
    Terminal,
    // TODO: mb RGB struct?
    /// hexadecimal RGB string such as ‘#ffffff’
    HEX(u32),
}

impl fmt::Display for Colour {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: Cow<'a, str> = match self {
            Self::Black => BLACK.into(),
            Self::Red => RED.into(),
            Self::Green => GREEN.into(),
            Self::Yellow => YELLOW.into(),
            Self::Blue => BLUE.into(),
            Self::Magenta => MAGENTA.into(),
            Self::Cyan => CYAN.into(),
            Self::White => WHITE.into(),
            Self::BrightRed => BRIGHTRED.into(),
            Self::BrightGreen => BRIGHTGREEN.into(),
            Self::BrightYellow => BRIGHTYELLOW.into(),
            Self::ColourSet256(n) => format!("colour{}", n).into(),
            Self::Default => DEFAULT.into(),
            Self::Terminal => TERMINAL.into(),
            Self::HEX(n) => format!("#{:06x}", n).into(),
        };
        write!(f, "{}", s)
    }
}

//use std::str::FromStr;
//
// TODO: ColourSet256, HEX
//impl FromStr for Colour {
//type Err = ();

//fn from_str(s: &str) -> Result<Self, Self::Err> {
//unimplemented!();
//match s {
//BLACK => Ok(Self::Black),
//RED => Ok(Self::Red),
//GREEN => Ok(Self::Green),
//BLUE => Ok(Self::Blue),
//MAGENTA => Ok(Self::Magenta),
//CYAN => Ok(Self::Cyan),
//WHITE => Ok(Self::White),
//BRIGHTRED => Ok(Self::BrightRed),
//BRIGHTGREEN => Ok(Self::BrightGreen),
//BRIGHTYELLOW => Ok(Self::BrightYellow),
//// ColourSet256
//DEFAULT => Ok(Self::Default),
//TERMINAL => Ok(Self::Terminal),
//// HEX
//_ => Err(()),
//}
//}
//}
