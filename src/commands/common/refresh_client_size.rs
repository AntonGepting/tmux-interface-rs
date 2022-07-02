use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg(feature = "tmux_3_3")]
pub struct RefreshClientSize {
    pub window_id: Option<usize>,
    pub width: usize,
    pub height: usize,
}

#[cfg(feature = "tmux_3_3")]
impl fmt::Display for RefreshClientSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.window_id {
            Some(window_id) => format!("@{}:{}x{}", window_id, self.width, self.height),
            None => format!("{}x{}", self.width, self.height),
        };

        write!(f, "{}", s)
    }
}
