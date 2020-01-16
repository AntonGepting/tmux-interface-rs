/// All functions from man tmux "Options" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS)
pub mod set_option;
#[cfg(feature = "tmux_2_6")]
pub mod set_window_option;
pub mod show_options;
pub mod show_window_options;
