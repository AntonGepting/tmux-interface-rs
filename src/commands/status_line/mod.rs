/// All functions from man tmux "Status line" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE)
#[cfg(feature = "tmux_1_0")]
pub mod command_prompt;
#[cfg(feature = "tmux_1_0")]
pub mod confirm_before;
#[cfg(feature = "tmux_3_0")]
pub mod display_menu;
#[cfg(feature = "tmux_1_0")]
pub mod display_message;
//#[cfg(feature = "tmux_1_0")]
//pub mod select_prompt;

#[cfg(feature = "tmux_1_0")]
pub mod command_prompt_tests;
#[cfg(feature = "tmux_1_0")]
pub mod confirm_before_tests;
#[cfg(feature = "tmux_3_0")]
pub mod display_menu_tests;
#[cfg(feature = "tmux_1_0")]
pub mod display_message_tests;
