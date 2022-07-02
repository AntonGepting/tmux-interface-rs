#[cfg(feature = "tmux_3_2")]
use std::borrow::Cow;

// TODO: enum for what?
/// [-B name:what:format]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg(feature = "tmux_3_2")]
pub struct Subscribe<'a> {
    pub name: Cow<'a, str>,
    /// empty to check the format only for the attached session
    /// pane ID such as ‘%0’; ‘%*’ for all panes in the attached session
    /// window ID such as ‘@0’; or ‘@*’ for all windows in the attached session
    pub what: Option<usize>,
    pub format: Option<usize>,
}
