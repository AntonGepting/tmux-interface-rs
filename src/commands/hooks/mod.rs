use crate::TmuxCommand;
#[cfg(feature = "tmux_2_2")]
use crate::{SetHook, ShowHooks};

/// All functions from man tmux "Hooks" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#HOOKS))
#[cfg(feature = "tmux_2_2")]
pub mod set_hook;
#[cfg(feature = "tmux_2_2")]
pub mod show_hooks;

#[cfg(feature = "tmux_2_2")]
pub mod set_hook_tests;
#[cfg(feature = "tmux_2_2")]
pub mod show_hooks_tests;

/// All functions from man tmux "Hooks" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#HOOKS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_2_2")]
    pub fn set_hook() -> SetHook<'a> {
        SetHook::new()
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn show_hooks() -> ShowHooks<'a> {
        ShowHooks::new()
    }
}

impl<'a> From<SetHook<'a>> for TmuxCommand<'a> {
    fn from(item: SetHook<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ShowHooks<'a>> for TmuxCommand<'a> {
    fn from(item: ShowHooks<'a>) -> Self {
        item.build()
    }
}
