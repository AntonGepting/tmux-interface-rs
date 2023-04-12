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
//!     use crate::tmux_interface::{Tmux, AttachSession, NewSession, TargetPane,
//!     TargetSession, KillSession, SendKeys, HasSession, TmuxCommand};
//!     
//!     let target_session = TargetSession::Raw("session_name").to_string();
//!
//!     let tmux = Tmux::new()
//!     .command(NewSession::new().detached().session_name(&target_session))
//!     .command(HasSession::new().target_session(&target_session))
//!     .command(AttachSession::new().target_session(&target_session))
//!     .command(SendKeys::new().key("exit").key("C-m"))
//!     .command(KillSession::new().target_session(&target_session))
//!     //.envs()
//!     .output()
//!     .unwrap();
//!
//!     ```
//!
//!
//! ```text
//! library in -> build command -> exec command -> recieve output -> parse output -> library out
//! user app -> library in -> ... -> library out -> user app
//! ```
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
//! let output = Tmux::new()
//!                .add_command(NewSession::new().detached().session_name("new_session_name1"))
//!                .add_command(AttachSession::new().target_session("new_session_name1"))
//!                .add_command(KillSession::new().target_session("new_session_name1"))
//!                .output()
//!                .unwrap();
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
//! use crate::tmux_interface::{TmuxCommand, Tmux, NewSession, KillSession};
//!
//! let mut tmux = Tmux::new();
//!                  .add_command(NewSession::new().detached().session_name("new_session_name3"))
//!                  .add_command(KillSession::new().target_session("new_session_name3"))
//!                  .output()
//!                  .unwrap();
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
//! use crate::tmux_interface::{TmuxCommand, Tmux, TargetSession};
//!
//! let mut tmux = TmuxCommand::new();
//! tmux.new_session().detached().session_name("new_session_default").output();
//! tmux.kill_session().target_session("new_session_default").output();
//! ```
//!
//! using `std::default::Default` trait:
//! ```
//! use crate::tmux_interface::{TmuxCommand, Tmux, NewSession, TargetSession};
//!
//! let mut tmux = TmuxCommand::new();
//! tmux.new_session().detached().session_name("new_session_default").output();
//! tmux.kill_session().target_session("new_session_default").output();
//! ```
//!
//! using direct structure modification:
//! ```
//! use crate::tmux_interface::{TmuxCommand, Tmux, NewSession, TargetSession};
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
//!
//! # Hierarchy
//!
//! ```
//!  +--------------------+
//!  | ServerOptionOutput |
//!  +--------------------+
//!
//!  +-----------------+
//!  | GetServerOption |
//!  +-----------------+
//!
//! 4. Command Builder
//!  +------+     +------------+      +---------------+        +-----+
//!  | Tmux |     | NewSession |      | AttachSession |        | ... |
//!  +------+     +------------+      +---------------+        +-----+
//!
//! 3. Tmux Command
//!  +-------------+                                  +------------+
//!  | TmuxCommand |                                  | TmuxOutput |
//!  +-------------+                                  +------------+
//!  +--------------+
//!  | TmuxCommands |
//!  +--------------+
//!
//! 2. cmd_builder library
//!  +------------------+
//!  | cmd_builder::Cmd |
//!  +------------------+
//!  +----------------------+
//!  | cmd_builder::CmdList |
//!  +----------------------+
//!
//! 1. Standard library
//!  +-----------------------+
//!  | std::process::Command |
//!  +-----------------------+
//!
//! 0. OS
//!  +--------+ +---------------+
//!  | fork() | | CreateProcess |
//!  +--------+ +---------------+
//! ```
//!
//!
//!
//!
//!
//! # Levels
//!
//! Tmux command invocation can be described and accessed on different levels:
//!
//! * 0. OS (`fork()`, `CreateProcess`, ... )
//! * 1. Standard Library ([`std::process::Command`])
//! * 2. Cmd Builder Library ([`cmd_builder::Cmd`], [`cmd_builder::CmdList`])
//! * 3. Tmux Interface Library - Command Builder ([`TmuxCommand`], [`TmuxCommands`])
//! * 4. Tmux Interface Library - Tmux Command Builder ([`Tmux`], [`NewSession`], [`AttachSession`] ... )
//!
//! and thereby:
//!
//! * Each level allows to build practically the same command, but with more or less effort and advantages
//! * Each level has some abstraction and some limitations
//! * Each level is based on top of the previous one
//!
//! ## Level Explanations and Examples
//!
//! * 0. syscall `fork(...)`, `CreateProcess(...)` - Operating System level abstraction
//!
//! * 1. [`std::process::Command`] - Rust standard library level abstraction
//!     * OS independence
//!     * comfortable working low level
//!     * manually build commands using literals
//!     * hard coded literals
//!
//!     ### Examples
//!     ```
//!     use std::process::Command;
//!
//!     let output = Command::new("tmux")
//!                   .args(["-2", "-uv", "new-session", "-A", "-DE", "-s", "my_session"])
//!                   .output()
//!                   .unwrap();
//!     ```
//!     ```text
//!     tmux -2 -uv new-session -A -DX -n my_session
//!     ```
//!
//! * 2. [`cmd_builder::Cmd`], [`cmd_builder::CmdList`] - custom Rust crate abstraction
//!     * additional functionality for [`std::process::Command`]
//!     * allows to store additional information about commands such as:
//!         * command alias (`new`), beside command name (`new-session`)
//!         * short flag name (`-l`) and long flag name (`--long-flag`)
//!         * custom separator, hyphen, etc... (` `, `-`, `--`, `=`, ``)
//!     * runtime mechanisms for deciding and building short or long commands
//!
//!     ### Examples
//!     ```
//!     use cmd_builder::Cmd;
//!
//!     let output = Cmd::new()
//!                    .name("tmux")
//!                    .push_flag_short('2')
//!                    .push_flag_short('u')
//!                    .push_flag_short('v')
//!                    .push_cmd(
//!                      Cmd::new()
//!                        .name("new-session")
//!                        .push_flag_short('D')
//!                        .push_flag_short('E')
//!                        .args(["-s", "my_session"]))
//!                    .combine_short_flags()
//!                    .output()
//!                    .unwrap();
//!     ```
//!     ```text
//!     tmux -2uv new-session -ADX -s my_session
//!     ```
//!
//! * 3. [`TmuxCommand`], [`TmuxCommands`] - just a wrapper around [`cmd_builder::Cmd`] inside `tmux_interface` crate,
//! same functionality
//!
//!     ### Examples
//!     ```
//!     use tmux_interface::TmuxCommand;
//!
//!     let output = TmuxCommand::new("tmux")
//!                    .push_flag_short('2')
//!                    .push_flag_short('u')
//!                    .push_flag_short('v')
//!                    .push_cmd(TmuxCommand::new("new-session")
//!                      .alias("new")
//!                      .push_flag_short('A')
//!                      .push_flag_short('D')
//!                      .push_flag_short('E')
//!                      .push_option("-s", "name"))
//!                    .output()
//!                    .unwrap();
//!     ```
//!     ```text
//!     tmux -2 -u -v new-session -A -D -E -s my_session
//!     ```
//!
//! * 4. [`Tmux`], [`NewSession`], [`AttachSession`] ... - tmux commands builder
//!     * structures, traits, implementations and methods as abstraction from literals
//!     * near to tmux naming as possible
//!     * build tmux commands
//!     * tmux commands can include binary name and arguments or nor for control mode
//!     * order of arguments doesn't matter
//!     * using macroses
//!
//!     ### Examples
//!     ```
//!     use tmux_interface::{Tmux, NewSession};
//!
//!     let output = Tmux::with_command(
//!                      NewSession::new()
//!                         .attach()
//!                         .detached()
//!                         .not_update_env()
//!                         .session_name("name"))
//!                    .colours256()
//!                    .force_utf8()
//!                    .verbose_logging()
//!                    .output()
//!                    .unwrap();
//!     ```
//!     or using macroses
//!     ```text
//!     let output = tmux!("-2", "-u", "-v", new_session!("-A", "-D", "-E", "-s my_session"))
//!                     .output()
//!                     .unwrap();
//!     ```
//!     ```text
//!     tmux -2 -u -v new-session -A -D -E -s my_session
//!     ```
//!
//!
//! * `Options` - tmux objects control
//!     * accessing and using internal tmux instances
//!         * formats
//!         * options
//!         * variables
//!         * ...
//!
//!     ### Example
//!     ```text
//!     unimplemented!();
//!     ```
//!
//! * `TmuxInterface` - tmux control
//!     * setting/getting methods abstraction, just an object with it's attributes
//!     * offline/online working (default/control mode)
//!     * mapping of whole tmux with it's all internal instances as an object in Rust
//!
//!     ### Example
//!     ```text
//!     unimplemented!();
//!     ```
//!
//!

extern crate cmd_builder;

pub mod commands;
pub mod control_mode;
pub mod error;
pub mod formats;
pub mod options;
pub mod styles;
pub mod target;
#[cfg(feature = "tmux_1_6")]
pub mod variables;

pub use commands::*;
pub use control_mode::*;
pub use error::Error;
pub use formats::*;
pub use options::*;
pub use styles::*;
pub use target::*;
#[cfg(feature = "tmux_1_6")]
pub use variables::*;
