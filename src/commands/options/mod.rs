use crate::{SetOption, SetWindowOption, ShowOptions, ShowWindowOptions, TmuxCommand};

/// All functions from man tmux "Options" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS)
#[cfg(feature = "tmux_1_0")]
pub mod set_option;
#[cfg(feature = "tmux_1_0")]
pub mod set_window_option;
#[cfg(feature = "tmux_1_0")]
pub mod show_options;
#[cfg(feature = "tmux_1_0")]
pub mod show_window_options;

//#[cfg(feature = "tmux_1_0")]
//pub mod set_option_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod set_window_option_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod show_options_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod show_window_options_tests;

impl<'a> TmuxCommand<'a> {
    pub fn set_option(&self) -> SetOption<'a> {
        SetOption::from(self)
    }

    pub fn set_window_option(&self) -> SetWindowOption<'a> {
        SetWindowOption::from(self)
    }

    pub fn show_options(&self) -> ShowOptions<'a> {
        ShowOptions::from(self)
    }

    pub fn show_window_options(&self) -> ShowWindowOptions<'a> {
        ShowWindowOptions::from(self)
    }
}
