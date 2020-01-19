/// All functions from man tmux "Status line" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE)
pub mod command_prompt;
pub mod confirm_before;
#[cfg(not(feature = "tmux_2_6"))]
pub mod display_menu;
pub mod display_message;

pub mod command_prompt_tests;
pub mod confirm_before_tests;
#[cfg(not(feature = "tmux_2_6"))]
pub mod display_menu_tests;
pub mod display_message_tests;
