// auto-generated file
//

/// All functions from man tmux "Global and session environment" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT))
///
use crate::TmuxCommand;

#[cfg(feature = "tmux_1_5")]
pub mod set_environment;
#[cfg(feature = "tmux_1_5")]
pub mod set_environment_macro;

#[cfg(feature = "tmux_1_5")]
pub mod show_environment;
#[cfg(feature = "tmux_1_5")]
pub mod show_environment_macro;

#[cfg(feature = "tmux_1_5")]
pub use set_environment::{SetEnv, SetEnvironment};

#[cfg(feature = "tmux_1_5")]
pub use show_environment::{ShowEnv, ShowEnvironment};

#[cfg(test)]
#[path = "."]
mod global_and_session_environment_tests {

    #[cfg(feature = "tmux_1_5")]
    mod set_environment_tests;

    #[cfg(feature = "tmux_1_5")]
    mod show_environment_tests;
}

/// All functions from man tmux "Global and session environment" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT))
///
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_1_5")]
    pub fn set_environment() -> SetEnvironment<'a> {
        SetEnvironment::new()
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn show_environment() -> ShowEnvironment<'a> {
        ShowEnvironment::new()
    }
}

#[cfg(feature = "tmux_1_5")]
impl<'a> From<SetEnvironment<'a>> for TmuxCommand<'a> {
    fn from(item: SetEnvironment<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_1_5")]
impl<'a> From<ShowEnvironment<'a>> for TmuxCommand<'a> {
    fn from(item: ShowEnvironment<'a>) -> Self {
        item.build()
    }
}
