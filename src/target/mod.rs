//! The [`target`][`crate::target`] module contains functions for building targets for tmux
//! commands
//!
//! * TargetPane
//!     * token (+, -, {...}) instead of name
//!     * index instead of name
//!     * id (%id) instead of name
//!     * exact name (=name)
//!     * start of a name
//!     * fn_match
//!
//! * TargetWindow
//!     * token (^, $, !, +, -) instead of name
//!     * index instead of name
//!     * id (@id) instead of name
//!     * exact name (=name)
//!     * start of a name
//!     * fn_match
//!
//! TargetSession
//!     * id ($id) instead of name
//!     * exact name (=name)
//!     * start of a name
//!     * fn_match
//!
//! # See Also
//! [Tmux Manual -> Commands](https://man7.org/linux/man-pages/man1/tmux.1.html#COMMANDS)
//!
//!
pub mod target_pane;
pub mod target_session;
pub mod target_window;

#[cfg(test)]
#[path = "."]
mod target_tests {
    pub mod target_pane_tests;
    pub mod target_session_tests;
    pub mod target_window_tests;
}
