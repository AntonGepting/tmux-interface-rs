//! The [`commands`][`crate::commands`] module contains functions for building and executing
//! tmux commands
//!
//! # On This Page
//! * 1. [Description](#1-description)
//! * 2. [Quick Start](#2-quick-start)
//! * 3. [Modules Overview](#4-modules-overview)
//!
//! * Command Builder
//!     * Direct Fields Initialization
//!     * Direct Instantiating
//!     * Using Builder Methods
//!     * Using Macros
//!     
//! Direct Initialization of Structure Fields
//! ```
//! use tmux_interface::Tmux;
//!
//! let mut tmux = Tmux::new();
//! tmux.verbose_logging = true;
//!
//! ```
//!
//! ```
//! use tmux_interface::Tmux;
//!
//! let tmux = Tmux {
//!     verbose_logging: true,
//!     ..Default::default()
//! };
//! ```
//!
//! Using Builder Methods
//!
//!
//! * Command Builder
//!     * One-liner
//!         * Autonomous
//!         * Binary
//!     * Multi-liner
//!         * Autonomous
//!         * Binary
//!
//! # Tmux Commands
//!
//! ## Example
//!
//! ```text
//! tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path]
//! [command [flags]]
//! ```
//!
//! * [`tmux`] - tmux binary command and it's arguments
//! * [`tmux_command`] - wrapper for [`std::process::Command`] type
//! * [`tmux_commands`] - wrapper for vector of [`std::process::Command`] type
//! * [`NewSession`] - tmux autonomous command
//!
//!
//!
//! ```text
//! NewSession::new() -> NewSession -> .build() -> TmuxCommand
//!
//! Tmux::new() -> Tmux -> .build() -> TmuxCommand
//! (only tmux can be passed to std::process::Command)
//! ```
//!
//! For convenience all tmux commands builder are splitted in sub-modules by the
//! similar principle as in tmux manual:
//! * [Buffers](buffers)
//! * [Client and Sessions](clients_and_sessions)
//! * [Global and Session Environment](global_and_session_environment)
//! * [Hooks](hooks)
//! * [Key Bindings](key_bindings)
//! * [Miscellaneous](miscellaneous)
//! * [Options](options)
//! * [Status Line](status_line)
//! * [Windows and Panes](windows_and_panes)
//!
//! # Overview
//!
//! Tmux commands
//! * single
//!     * autonomous
//!     * binary
//! * multiple
//!     * autonomous
//!     * binary
//!
//!
//! autonomous/binary - tmux commands can be called from outside of tmux (building one command
//! sequence with tmux binary name) or from inside of tmux
//!
//!
//! multiple/single - tmux commands can be called as a single one or can be chained (similar as in shell)
//!
//!
//! # Single autonomous tmux command
//!
//! Single tmux commands can be separated in two types:
//!
//! * **autonomous tmux command**, just a command itself and it's arguments, used for invoking from
//! inside of tmux
//!
//! ## Example
//!
//! ```text
//! new-session -d -n name
//! ```
//!
//! ```
//! use tmux_interface::NewSession;
//!
//! let new_session = NewSession::new().detached().session_name("my_session").build();
//! ```
//!
//! * **binary tmux command**, a command including tmux binary name and it's arguments used for
//! invoking from outside of tmux
//!
//! ## Example
//!
//! ```text
//! tmux -v new-session -d -n name
//! ```
//!
//! ```
//! use tmux_interface::{Tmux, NewSession};
//!
//! let tmux = Tmux::with_command(
//!              NewSession::new()
//!                .detached()
//!                .session_name("my_session"))
//!              .verbose_logging()
//!              .build();
//! ```
//!
//! # Multiple tmux commands
//!
//! And multiple tmux commands can be combined:
//!
//! * multiple tmux commands
//!
//! ## Example
//!
//! ```text
//! new-session -d -n name ; attach-session -t name
//! ```
//!
//! ```
//! use tmux_interface::TmuxCommands;
//!
//! let cmds = TmuxCommands::new();
//! ```
//!
//! * multiple tmux binary commands
//!
//! ## Example
//!
//! ```text
//! tmux new-session -d -n name ; tmux attach-session -t name
//! ```
//!
//! ```
//! use tmux_interface::{Tmux, TmuxCommands};
//!
//! let cmds = Tmux::new().verbose_logging().commands(TmuxCommands::new());
//! ```
//!
//! # See Also:
//! * [Tmux Manual -> Commands](https://man7.org/linux/man-pages/man1/tmux.1.html#COMMANDS)
//!
pub mod common;

pub mod constants;

pub mod tmux;
pub mod tmux_macro;

pub mod tmux_command;
pub mod tmux_commands;
pub mod tmux_output;

#[cfg(test)]
#[path = "."]
mod commands_tests {
    mod tmux_command_tests;
    mod tmux_commands_tests;
    mod tmux_tests;
}

pub mod buffers;
pub mod clients_and_sessions;
pub mod global_and_session_environment;
pub mod hooks;
pub mod key_bindings;
pub mod miscellaneous;
pub mod options;
pub mod status_line;
pub mod windows_and_panes;

pub use common::*;

// buffers
pub use buffers::*;
// clients and sessions
pub use clients_and_sessions::*;
// global and session environment
pub use global_and_session_environment::*;
// hooks
pub use hooks::*;
// key bindings
pub use key_bindings::*;
// miscellaneous
pub use miscellaneous::*;
// options
pub use options::*;
// status line
pub use status_line::*;
// windows and panes
pub use windows_and_panes::*;

pub use tmux::Tmux;
pub use tmux_command::TmuxCommand;
pub use tmux_commands::TmuxCommands;
pub use tmux_output::TmuxOutput;

// XXX: ?
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html))
impl<'a> TmuxCommand<'a> {
    pub fn tmux() -> Tmux<'a> {
        Tmux::new()
    }
}
