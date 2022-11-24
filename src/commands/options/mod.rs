use crate::TmuxCommand;

/// All functions from man tmux "Options" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS))
#[cfg(feature = "tmux_0_8")]
pub mod set_option;
// FIXME
//#[cfg(feature = "tmux_0_8")]
//pub mod set_option_macro;

#[cfg(feature = "tmux_0_8")]
pub mod set_window_option;
// FIXME
//#[cfg(feature = "tmux_0_8")]
//pub mod set_window_option_macro;

#[cfg(feature = "tmux_0_8")]
pub mod show_options;
#[cfg(feature = "tmux_0_8")]
pub mod show_options_macro;

#[cfg(feature = "tmux_0_8")]
pub mod show_window_options;
#[cfg(feature = "tmux_0_8")]
pub mod show_window_options_macro;

#[cfg(feature = "tmux_0_8")]
pub use set_option::SetOption;
#[cfg(feature = "tmux_0_8")]
pub use set_window_option::SetWindowOption;
#[cfg(feature = "tmux_0_8")]
pub use show_options::ShowOptions;
#[cfg(feature = "tmux_0_8")]
pub use show_window_options::ShowWindowOptions;

#[cfg(test)]
#[path = "."]
mod options_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod set_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod set_window_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod show_options_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod show_window_options_tests;
}

/// All functions from man tmux "Options" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn set_option() -> SetOption<'a> {
        SetOption::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn set_window_option() -> SetWindowOption<'a> {
        SetWindowOption::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn show_options() -> ShowOptions<'a> {
        ShowOptions::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn show_window_options() -> ShowWindowOptions<'a> {
        ShowWindowOptions::new()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<SetOption<'a>> for TmuxCommand<'a> {
    fn from(item: SetOption<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<SetWindowOption<'a>> for TmuxCommand<'a> {
    fn from(item: SetWindowOption<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<ShowOptions<'a>> for TmuxCommand<'a> {
    fn from(item: ShowOptions<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<ShowWindowOptions<'a>> for TmuxCommand<'a> {
    fn from(item: ShowWindowOptions<'a>) -> Self {
        item.build()
    }
}
