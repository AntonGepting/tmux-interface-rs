use crate::TmuxCommand;
#[cfg(feature = "tmux_0_8")]
use crate::{SetOption, SetWindowOption, ShowOptions, ShowWindowOptions};

/// All functions from man tmux "Options" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS))
#[cfg(feature = "tmux_0_8")]
pub mod set_option;
#[cfg(feature = "tmux_0_8")]
pub mod set_window_option;
#[cfg(feature = "tmux_0_8")]
pub mod show_options;
#[cfg(feature = "tmux_0_8")]
pub mod show_window_options;

#[cfg(feature = "tmux_0_8")]
pub mod set_option_tests;
#[cfg(feature = "tmux_0_8")]
pub mod set_window_option_tests;
#[cfg(feature = "tmux_0_8")]
pub mod show_options_tests;
#[cfg(feature = "tmux_0_8")]
pub mod show_window_options_tests;

/// All functions from man tmux "Options" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn set_option(&self) -> SetOption<'a> {
        SetOption::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn set_window_option(&self) -> SetWindowOption<'a> {
        SetWindowOption::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn show_options(&self) -> ShowOptions<'a> {
        ShowOptions::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn show_window_options(&self) -> ShowWindowOptions<'a> {
        ShowWindowOptions::from(self)
    }
}
