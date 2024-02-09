//! The [`variables`][`crate::variables`] module contains functions for getting variables from tmux
//!
//! Getting variables
//! * session
//! * window
//! * pane
//! * layout
//! * client
//!
//! # See Also
//! * [Formats][`crate::formats`]
//! * [Tmux Manual -> Formats](https://man7.org/linux/man-pages/man1/tmux.1.html#FORMATS)
pub mod buffer;
pub mod client;
pub mod layout;
pub mod misc;
pub mod pane;
pub mod session;
pub mod window;

pub use buffer::*;
pub use client::*;
pub use layout::*;
pub use misc::*;
pub use pane::*;
pub use session::*;
pub use window::*;
