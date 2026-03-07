use std::fmt;

// XXX: size? similar struct XxY, rect, mb rename
#[cfg(feature = "tmux_3_2")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Size {
    // XXX: Size:Size?
    Size(usize),
    Percentage(usize),
}

#[cfg(feature = "tmux_3_2")]
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Size::Size(size) => size.to_string(),
            Size::Percentage(size) => {
                format!("{}%", size)
            }
        };

        write!(f, "{}", s)
    }
}
