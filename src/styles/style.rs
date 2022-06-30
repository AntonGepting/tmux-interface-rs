use super::{Align, Colour, List, Range};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Style {
    /// Set the foreground colour
    Fg(Colour),
    /// Set the background colour
    Bg(Colour),
    /// Set no attributes (turn off any active attributes)
    // right name: None
    NoStyle,
    /// Set an attribute.
    /// Any of the attributes may be prefixed with ‘no’ to unset.
    /// acs is the terminal alternate character set.
    Acs,
    NoAcs,
    Bright,
    NoBright,
    Bold,
    NoBold,
    Dim,
    NoDim,
    Underscore,
    NoUnderscore,
    Blink,
    NoBlink,
    Reverse,
    NoReverse,
    Hidden,
    NoHidden,
    Italics,
    NoItalics,
    Overline,
    NoOverline,
    Strikethrough,
    NoStrikethrough,
    DoubleUnderscore,
    NoDoubleUnderscore,
    CurlyUnderscore,
    NoCurlyUnderscore,
    DottedUnderscore,
    NoDottedUnderscore,
    DashedUnderscore,
    NoDashedUnderscore,
    Align(Align),
    NoAlign,
    /// Fill the available space with a background colour if appropriate
    Fill(Colour),
    /// list
    List(List),
    NoList,
    //
    PushDefault,
    PopDefault,
    //
    Range(Range),
    NoRange,
}

impl fmt::Display for Style {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match self {
            Self::Fg(colour) => format!("fg={}", colour.to_string()),
            Self::Bg(colour) => format!("bg={}", colour.to_string()),
            // right name: None
            Self::NoStyle => "none".to_string(),
            Self::Acs => "acs".to_string(),
            Self::NoAcs => "noacs".to_string(),
            Self::Bright => "bright".to_string(),
            Self::NoBright => "nobright".to_string(),
            Self::Bold => "bold".to_string(),
            Self::NoBold => "nobold".to_string(),
            Self::Dim => "dim".to_string(),
            Self::NoDim => "nodim".to_string(),
            Self::Underscore => "underscore".to_string(),
            Self::NoUnderscore => "nounderscore".to_string(),
            Self::Blink => "blink".to_string(),
            Self::NoBlink => "noblink".to_string(),
            Self::Reverse => "reverse".to_string(),
            Self::NoReverse => "noreverse".to_string(),
            Self::Hidden => "hidden".to_string(),
            Self::NoHidden => "nohidden".to_string(),
            Self::Italics => "italics".to_string(),
            Self::NoItalics => "noitalics".to_string(),
            Self::Overline => "overline".to_string(),
            Self::NoOverline => "nooverline".to_string(),
            Self::Strikethrough => "strikethrough".to_string(),
            Self::NoStrikethrough => "nostrikethrough".to_string(),
            Self::DoubleUnderscore => "double-underscore".to_string(),
            Self::NoDoubleUnderscore => "nodouble-underscore".to_string(),
            Self::CurlyUnderscore => "curly-underscore".to_string(),
            Self::NoCurlyUnderscore => "nocurly-underscore".to_string(),
            Self::DottedUnderscore => "dotted-underscore".to_string(),
            Self::NoDottedUnderscore => "nodotted-underscore".to_string(),
            Self::DashedUnderscore => "dashed-underscore".to_string(),
            Self::NoDashedUnderscore => "nodashed-underscore".to_string(),
            Self::Align(align) => format!("align={}", align.to_string()),
            Self::NoAlign => "noalign".to_string(),
            Self::Fill(colour) => format!("fill={}", colour.to_string()),
            Self::List(list) => format!("list={}", list.to_string()),
            Self::NoList => "nolist".to_string(),
            Self::PushDefault => "push-default".to_string(),
            Self::PopDefault => "pop-default".to_string(),
            Self::Range(range) => format!("range={}", range.to_string()),
            Self::NoRange => "norange".to_string(),
        };
        write!(f, "{}", s)
    }
}
