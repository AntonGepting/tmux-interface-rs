#[cfg(feature = "tmux_3_2")]
use std::borrow::Cow;

#[cfg(feature = "tmux_3_2")]
use crate::State;

// XXX: struct or tuple?
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg(feature = "tmux_3_2")]
pub struct AllowActions<'a> {
    pub pane: Cow<'a, str>,
    pub state: State,
}
