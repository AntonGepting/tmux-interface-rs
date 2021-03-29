use crate::TmuxCommand;

#[cfg(feature = "tmux_1_0")]
use crate::{SetEnvironment, ShowEnvironment};

/// All functions from man tmux "Global and session environment" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT))
#[cfg(feature = "tmux_1_0")]
pub mod set_environment;
#[cfg(feature = "tmux_1_0")]
pub mod show_environment;

#[cfg(feature = "tmux_1_0")]
pub mod set_environment_tests;
#[cfg(feature = "tmux_1_0")]
pub mod show_environment_tests;

/// All functions from man tmux "Global and session environment" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_1_0")]
    pub fn set_environment(&self) -> SetEnvironment<'a> {
        SetEnvironment::from(self)
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn show_environment(&self) -> ShowEnvironment<'a> {
        ShowEnvironment::from(self)
    }
}
