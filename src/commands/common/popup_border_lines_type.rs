#[cfg(feature = "tmux_3_3")]
use std::fmt;

/// Set the type of characters used for drawing popup borders. type may be one of:
/// ‘double’ and ‘heavy’ will fall back to standard ACS line drawing when UTF-8 is not supported.
#[cfg(feature = "tmux_3_3")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PopupBorderLinesType {
    /// single lines using ACS or UTF-8 characters (default)
    Single,
    /// variation of single with rounded corners using UTF-8 characters
    Rounded,
    /// double lines using UTF-8 characters
    Double,
    /// heavy lines using UTF-8 characters
    Heavy,
    /// simple ASCII characters
    Simple,
    /// simple ASCII space character
    Padded,
    /// no border
    NoBorder,
}

#[cfg(feature = "tmux_3_3")]
impl fmt::Display for PopupBorderLinesType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Single => "single",
            Self::Rounded => "rounded",
            Self::Double => "double",
            Self::Heavy => "heavy",
            Self::Simple => "simple",
            Self::Padded => "padded",
            Self::NoBorder => "none",
        };
        write!(f, "{}", s)
    }
}

