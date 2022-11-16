use crate::TmuxCommand;

/// All functions from man tmux "Hooks" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#HOOKS))
#[cfg(feature = "tmux_2_2")]
pub mod set_hook;
#[cfg(feature = "tmux_2_2")]
pub mod set_hook_macro;

#[cfg(feature = "tmux_2_2")]
pub mod show_hooks;
#[cfg(feature = "tmux_2_2")]
pub mod show_hooks_macro;

#[cfg(feature = "tmux_2_2")]
pub use set_hook::SetHook;
#[cfg(feature = "tmux_2_2")]
pub use show_hooks::ShowHooks;

#[cfg(test)]
#[path = "."]
mod hooks_tests {
    #[cfg(feature = "tmux_2_2")]
    pub mod set_hook_tests;
    #[cfg(feature = "tmux_2_2")]
    pub mod show_hooks_tests;
}

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

#[cfg(feature = "tmux_2_2")]
impl<'a> From<SetHook<'a>> for TmuxCommand<'a> {
    fn from(item: SetHook<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_2_2")]
impl<'a> From<ShowHooks<'a>> for TmuxCommand<'a> {
    fn from(item: ShowHooks<'a>) -> Self {
        item.build()
    }
}
