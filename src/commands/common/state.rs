#[cfg(feature = "tmux_3_2")]
use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg(feature = "tmux_3_2")]
pub enum State {
    On,
    Off,
    Continue,
    Pause,
}

#[cfg(feature = "tmux_3_2")]
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Continue => "continue",
            Self::Pause => "pause",
        };
        write!(f, "{}", s)
    }
}
