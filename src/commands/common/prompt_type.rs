#[cfg(feature = "tmux_3_3")]
use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg(feature = "tmux_3_3")]
pub enum PromptType {
    /// `command`
    Command,
    /// `search`
    Search,
    /// `target`
    Target,
    /// `window-target`
    WindowTarget,
}

#[cfg(feature = "tmux_3_3")]
impl fmt::Display for PromptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            PromptType::Command => "command",
            PromptType::Search => "search",
            PromptType::Target => "target",
            PromptType::WindowTarget => "window-target",
        };

        write!(f, "{}", s)
    }
}
