//! The [`control_mode`][`crate::control_mode`] module contains functions for working in
//! control mode of tmux
//!
//! # See Also
//! * [Tmux Manual -> Control Mode](https://man7.org/linux/man-pages/man1/tmux.1.html#CONTROL_MODE)
//!
pub mod constants;
#[cfg(feature = "tmux_1_8")]
pub mod control_mode;

pub mod control_mode_ctl;

#[cfg(test)]
#[cfg(feature = "tmux_1_8")]
pub mod control_mode_tests;
