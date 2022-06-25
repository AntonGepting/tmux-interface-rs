use crate::TmuxCommand;

#[cfg(feature = "tmux_1_0")]
use crate::{SetEnvironment, ShowEnvironment};

/// All functions from man tmux "Global and session environment" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT))
#[cfg(feature = "tmux_1_0")]
pub mod set_environment;
#[cfg(feature = "tmux_1_0")]
pub mod show_environment;

#[cfg(test)]
#[path = "."]
mod global_and_session_environment_tests {
    #[cfg(feature = "tmux_1_0")]
    pub mod set_environment_tests;
    #[cfg(feature = "tmux_1_0")]
    pub mod show_environment_tests;
}

/// All functions from man tmux "Global and session environment" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_1_0")]
    pub fn set_environment() -> SetEnvironment<'a> {
        SetEnvironment::new()
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn show_environment() -> ShowEnvironment<'a> {
        ShowEnvironment::new()
    }
}

impl<'a> From<SetEnvironment<'a>> for TmuxCommand<'a> {
    fn from(item: SetEnvironment<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ShowEnvironment<'a>> for TmuxCommand<'a> {
    fn from(item: ShowEnvironment<'a>) -> Self {
        item.build()
    }
}
