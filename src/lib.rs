#![forbid(unsafe_code)]
//#![warn(missing_docs)]

//! `tmux_interface` is a library for communication with [TMUX](https://github.com/tmux/tmux) via
//! CLI.
//!
//! # Description
//!
//! Main purpose of the `tmux_interface` library is to implement simple sending and recieving data
//! mechanisms for intercommunication with `TMUX` only via standard streams (`stdin`, `stdout`).
//!
//! ## Usage / Quick Start
//!
//! 1. Add a dependency in your `Cargo.toml`. Versions below `0.1.0` are
//!    mostly for development and testing purposes (further versions may have
//!    different ABI, use them in your projects on your own risk).
//!
//!     ```text
//!     [dependencies]
//!     tmux_interface = "^0.1.0"
//!     ```
//!
//!     You can also add `features` to your dependencies entry in `Cargo.toml`, if
//!     you want to specify the version of tmux you want to use. Different tmux
//!     versions may have incompatible CLI changes. Following `features` are currently
//!     supported:
//!
//!     - `tmux_X_X` - tmux latest, default (based on tmux master branch)
//!     - `tmux_2_6` - tmux 2.6 (included in Ubuntu 18.04 LTS Bionic Beaver)
//!     <!--- `tmux_2_1` - tmux 2.1 (included in Ubuntu 16.04 LTS Xenial Xerus) -->
//!     <!--- `tmux 1_8` - tmux 1.8 (included in Ubuntu 14.04 LTS Trusty Tahr) -->
//!     <!--- `tmux_1_6` - tmux 1.6 (included in Ubuntu 12.04 LTS Precise Pangolin)-->
//!
//!     ```text
//!     [dependencies]
//!     tmux_interface = { version = "^0.1.0", features = ["tmux_2_6"] }
//!     ```
//!
//!     by default `tmux_X_X` is used. It can be removed with `--no-default-features`
//!     cargo command line option or with `default-features = false` option in `Cargo.toml`
//!
//!     ```text
//!     [dependencies]
//!     tmux_interface = { version = "^0.1.0", default-features = false, features = ["tmux_2_6"] }
//!     ```
//!
//! <!--Add local repository-->
//! <!--```-->
//! <!--[dependencies]-->
//! <!--tmux_interface = { version = "0.0.7", path = "../tmux-interface", features = ["tmux_2_6"] }-->
//! <!--```-->
//!
//! <!--```-->
//! <!--Add remote repository-->
//! <!--tmux_interface = { git = "https://github.com/AntonGepting/tmux-interface-rs.git", branch = "dev" }-->
//! <!--```-->
//!
//!
//! 2. Add extern crate and use in your source file.
//!     ```
//!     extern crate tmux_interface;
//!     ```
//!
//! 3. Use it's functions
//!     ```
//!     use crate::tmux_interface::{AttachSession, NewSession, TargetPane, TargetSession, TmuxCommand};
//!
//!     let target_session = TargetSession::Raw("session_name").to_string();
//!
//!     let tmux = Tmux::new()
//!     .command(NewSession().detached().session_name(target_session))
//!     .command(HasSession().target_session(target_session))
//!     .command(AttachSession::new().target_session(target_session))
//!     .command(SendKeys::new().key("exit").key("C-m"))
//!     .command(KillSession().target_session(target_session))
//!     //.envs()
//!     .output()
//!     .unwrap();
//!
//!     ```
//!
//! # Overview
//!
//! * Commands ([`commands`])
//!     * Clients and Sessions ([`clients_and_sessions`](crate::commands::clients_and_sessions))
//!     * Windows and Panes ([`windows_and_panes`](crate::commands::windows_and_panes))
//!     * Key Bindings ([`key_bindings`](crate::commands::key_bindings))
//!     * Options ([`options`](crate::commands::options))
//!     * Hooks ([`hooks`](crate::commands::hooks))
//!     * Global and Session Environment ([`global_and_session_environment`](crate::commands::global_and_session_environment))
//!     * Status Line ([`status_line`](crate::commands::status_line))
//!     * Buffers ([`buffers`](crate::commands::buffers))
//!     * Miscellaneous ([`miscellaneous`](crate::commands::miscellaneous))
//!     * Common ([`common`](crate::commands::common))
//!
//! * Variables ([`variables`])
//!     * [`Sessions`](crate::Sessions)
//!     * [`Session`](crate::Session)
//!     * [`Windows`](crate::Windows)
//!     * [`Window`](crate::Window)
//!     * [`Panes`](crate::Panes)
//!     * [`Pane`](crate::Pane)
//!     * ...
//!
//! * Formats ([`formats`])
//!     * [`Formats`][crate::formats::Formats]
//!     * [`FormatsOutput`][crate::formats::FormatsOutput]
//!     * [`Variable`][crate::formats::Variable]
//!     * [`VariableOutput`][crate::formats::VariableOutput]
//!
//!
//! * Styles ([`styles`])
//!     * [`StyleList`][crate::styles::StyleList]
//!     * [`Style`][crate::styles::Style]
//!
//! * Target ([`target`])
//!     * [`TargetSession`]
//!     * [`TargetWindow`]
//!     * [`TargetPane`]
//!
//! Main structure is [`TmuxCommand`](crate::TmuxCommand) wich has all these wrapper functions implementations.
//! This goal can be reached by splitting it into two separate tasks:
//!
//! 1. Providing wrapper functions for tmux subcommands (which is sending data). Wrapper functions are
//! structured like in tmux manual in few next categories:
//!
//! 2. Parsing functions for tmux output as rust structures (which is recieving data). Parsing function are
//! structured by objects they operate with:
//!
//! # Conventions
//!
//! Library Functions:
//!     - Function names and their grouping are inherited from tmux manual
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::{AttachSession, NewSession, KillSession, Tmux};
//!
//! let tmux = Tmux::new()
//! .command(NewSession::new().detached().session_name("new_session_name1"))
//! .command(AttachSession::new().target_session("new_session_name1"))
//! .command(KillSession::new().target_session("new_session_name1"))
//! .output()
//! .unwrap();
//! ```
//!
//! # Examples
//!
//! Parsing examples:
//! ```
//! use crate::tmux_interface::{Sessions, Session, Windows, Window, Pane, Panes, TargetSession,
//! TargetWindowExt};
//!
//! let sessions = Sessions::get().unwrap();
//! let windows = Windows::get(&TargetSession::Raw("0")).unwrap();
//! let panes = Panes::get(&TargetWindowExt::raw("0:1")).unwrap();
//! ```
//!
//!
//! # Examples
//!
//! Change tmux command line flags, options
//! ```
//! use crate::tmux_interface::{TmuxCommand, NewSession, KillSession};
//!
//! let mut tmux = Tmux::new().bin("tmux")
//! .command(NewSession::new().detached().session_name("new_session_name3"))
//! .command(KillSession::new().target_session("new_session_name3"))
//! .output()
//! .unwrap();
//! ```
//!
//! # New session
//!
//! ## Examples
//! Create a new tmux session without any additional parameters (alternative to: `tmux new-session`)
//!
//! ```text
//! use crate::tmux_interface::TmuxCommand;
//!
//! let mut tmux = TmuxCommand::new();
//! tmux.new_session().output().unwrap();
//! ```
//!
//! ## Examples
//!
//! Create a new tmux session with some additional parameters (alternative to: `tmux new -d -s new_session`)
//! using builder pattern:
//!
//! ```
//! use crate::tmux_interface::{TmuxCommand, TargetSession};
//!
//! let mut tmux = Tmux::new();
//! tmux.new_session().detached().session_name("new_session_default").output();
//! tmux.kill_session().target_session("new_session_default").output();
//! ```
//!
//! using `std::default::Default` trait:
//! ```
//! use crate::tmux_interface::{TmuxCommand, NewSession, TargetSession};
//!
//! let mut tmux = Tmux::new();
//! let new_session =
//! tmux.new_session().detached().session_name("new_session_default").output();
//! tmux.kill_session().target_session("new_session_default").output();
//! ```
//!
//! using direct structure modification:
//! ```
//! use crate::tmux_interface::{TmuxCommand, NewSession, TargetSession};
//!
//! let mut tmux = Tmux::new();
//! let mut new_session = NewSession::new();
//! new_session.detached();
//! new_session.session_name("new_session_direct");
//! new_session.output();
//! tmux.kill_session().target_session("new_session_direct").output();
//! ```
//!
//!
//! ## Library levels
//!
//! Invokation process can be described by few levels.
//! * Each level allows to do the same, but with more or less impact and advantage
//! * Each level has some abstraction and some limitations
//! * Each level is based on top of the previous one
//!
//! Levels:
//!
//! 0. syscall `fork(...)`, `CreateProcess(...)` - Operating System abstraction
//!
//! 1. [`std::process::Command`] - Rust standard library abstraction
//!     * OS independence
//!     * comfortable working low level
//!     * manually build commands using strings and string arrays types
//!
//!     ### Example:
//!     ```
//!     let cmd = Command::new("tmux")
//!                 .arg("new-session")
//!                 .args(["-s", "name"])
//!                 .output();
//!     ```
//!
//! 2. [`cmd_builder::Cmd`], [`cmd_builder::CmdList`] - custom Rust crate abstraction
//!     * additional functionality for [`std::process::Command`]
//!     * allows to store alternative information about commands such as:
//!         * command name `new-session` and command alias `new`
//!         * short flag name `-l` and long flag name `--long-flag`
//!         * custom separator, hyphen, etc... (` `, `-`, `--`, `=`, ``)
//!
//!     * allows runtime mechanisms for deciding and building short or long commands
//!
//!     ### Example:
//!     ```
//!     let cmd = Cmd::new("tmux")
//!                 .subcommand("new-session")
//!                 .alias("new")
//!                 .option("-s", "name")
//!                 .output();
//!     ```
//!
//! 3. [`TmuxCommand`], [`TmuxCommands`] - just a wrapper around [`cmd_builder::Cmd`] inside `tmux_interface` crate,
//! same functionality
//!
//!     ### Example:
//!     ```
//!     let cmd = TmuxCommand::new("tmux")
//!                 .subcommand("new-session")
//!                 .alias("new")
//!                 .option("-s", "name")
//!                 .output();
//!     ```
//!
//! 4. [`Tmux`], [`NewSession`], [`AttachSession`] ... - tmux commands builder
//!     * near tmux as possible
//!     * build tmux commands
//!     * tmux commands can include binary name and arguments or nor for control mode
//!     * names near tmux
//!     * order of arguments doesn't matter
//!
//!     ### Example:
//!     ```
//!     let cmd = Tmux::new()
//!                 .command(NewSession::new()
//!                 .session_name("name"))
//!                 .output();
//!     ```
//!
//!
//! 5. [`Options`] - tmux objects control
//!     * accessing and using internal tmux instances
//!         * formats
//!         * options
//!         * variables
//!         * ...
//!
//!     ### Example
//!     ```
//!     unimplemented!();
//!     ```
//!
//! 6. `TmuxInterface` - tmux control
//!     * setting/getting methods abstraction, just an object with it's attributes
//!     * offline/online working (default/control mode)
//!     * mapping of whole tmux with it's all internal instances as an object in Rust
//!
//!     ### Example
//!     ```
//!     unimplemented!();
//!     ```
//!
//!

extern crate cmd_builder;

pub mod error;

pub mod commands;
pub mod options;
pub mod target;
#[cfg(feature = "tmux_1_6")]
pub mod variables;

pub mod version;

pub mod control_mode;

pub mod formats;

pub mod styles;

// options module
pub use options::*;

// commands module
pub use commands::*;

// target module
pub use target::*;

// variables module
pub use variables::*;

pub use version::Version;

// structs
pub use self::error::Error;

#[cfg(test)]
#[path = "."]
mod lib_tests {
    //mod options_tests;
    //mod tmux_option_tests;
    mod version_tests;
}
