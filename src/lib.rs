#![forbid(unsafe_code)]
//#![warn(missing_docs)]

//! `tmux_interface` is a library for communication with [TMUX](https://github.com/tmux/tmux) via
//! CLI.
//!
//! # On This Page
//! * 1. [Description](#1-description)
//! * 2. [Quick Start](#2-quick-start)
//! * 3. [Package Compilation Features](#3-package-compilation-features)
//!     * 3.1. [Tmux Version](#31-tmux-version)
//!     * 3.2. [Tmux Command Alias](#32-tmux-command-alias)
//!     * 3.3. [Repository](#3-3-repository)
//!         * 3.3.1 [Using Crates Repository](#331-using-crates-repository)
//!         * 3.3.2 [Using Local Repository](#332-using-local-repository)
//!         * 3.3.3 [Using Remote Repository](#333-using-remote-repository)
//! * 4. [Modules Overview](#4-modules-overview)
//!
//! # 1. Description
//!
//! Main purpose of the `tmux_interface` library is to implement simple sending and receiving data
//! mechanisms for intercommunication with `TMUX` only via standard streams (`stdin`, `stdout`, `stderr`).
//!
//! # 2. Quick Start
//!
//! 1. Add a dependency in your `Cargo.toml`. Versions below `1.0.0` are
//!    mostly for development and testing purposes (use them in your projects on
//!    your own risk, further versions may have different API).
//!
//!     ```text
//!     [dependencies]
//!     tmux_interface = "1.0.0"
//!     ```
//!
//! 2. Add extern crate in your source file.
//!     ```
//!     extern crate tmux_interface;
//!     ```
//!
//! 3. Use it's functions
//!
//!     ### Example 1
//!     ```
//!     use tmux_interface::{HasSession, KillSession, NewSession, NewWindow, SplitWindow, Tmux};
//!
//!     let target_session = "example_1";
//!
//!     // tmux new -d -s example_1 ; neww ; splitw -v
//!     Tmux::new()
//!         .add_command(NewSession::new().detached().session_name(target_session))
//!         .add_command(NewWindow::new())
//!         .add_command(SplitWindow::new().vertical())
//!         .output()
//!         .unwrap();
//!
//!     // tmux has -t example_1
//!     let status = Tmux::with_command(HasSession::new().target_session(target_session))
//!         .status()
//!         .unwrap()
//!         .success();
//!
//!     assert!(status);
//!
//!     // tmux kill-session -t example_1
//!     Tmux::with_command(KillSession::new().target_session(target_session))
//!         .output()
//!         .unwrap();
//!
//!     ```
//!
//! Call chain, data flow
//! ```text
//! library in -> build command -> exec command -> receive output -> parse output -> library out
//! user app -> library in -> ... -> library out -> user app
//! ```
//!
//! # 3. Package Compilation Features
//!
//! ## 3.1 Tmux Version
//!
//! Different tmux versions may have incompatible CLI changes.
//! Following versions features are currently supported:
//!
//! **Table:** Cargo.toml features, tmux versions list
//!
//! | Feature Name  | Tmux Version  | CI Tests | Comment                                     |
//! |---------------|---------------|----------|---------------------------------------------|
//! | `tmux_0_8`    | `tmux 0.8`    |          |                                             |
//! | `tmux_0_9`    | `tmux 0.9`    |          |                                             |
//! | `tmux_1_0`    | `tmux 1.0`    |          |                                             |
//! | `tmux_1_1`    | `tmux 1.1`    |          |                                             |
//! | `tmux_1_2`    | `tmux 1.2`    |          |                                             |
//! | `tmux_1_3`    | `tmux 1.3`    |          |                                             |
//! | `tmux_1_4`    | `tmux 1.4`    |          |                                             |
//! | `tmux_1_5`    | `tmux 1.5`    |          |                                             |
//! | `tmux_1_6`    | `tmux 1.6`    |          | Ubuntu 11.04 LTS Precise Pangolin, CentOS 6 |
//! | `tmux_1_7`    | `tmux 1.7`    |          | Ubuntu 14.04 LTS Trusty Tahr, CentOS 7      |
//! | `tmux_1_8`    | `tmux 1.8`    |        x |                                             |
//! | `tmux_1_9`    | `tmux 1.9`    |        x | Debian Jessie                               |
//! | `tmux_1_9a`   | `tmux 1.9a`   |        x |                                             |
//! | `tmux_2_0`    | `tmux 2.0`    |        x |                                             |
//! | `tmux_2_1`    | `tmux 2.1`    |        x | Ubuntu 16.04 LTS Xenial Xerus               |
//! | `tmux_2_2`    | `tmux 2.2`    |        x |                                             |
//! | `tmux_2_3`    | `tmux 2.3`    |        x | Debian Stretch                              |
//! | `tmux_2_4`    | `tmux 2.4`    |        x |                                             |
//! | `tmux_2_5`    | `tmux 2.5`    |        x |                                             |
//! | `tmux_2_6`    | `tmux 2.6`    |        x | Ubuntu 18.04 LTS Bionic Beaver              |
//! | `tmux_2_7`    | `tmux 2.7`    |        x | CentOS 8                                    |
//! | `tmux_2_8`    | `tmux 2.8`    |        x | Debian Buster                               |
//! | `tmux_2_9`    | `tmux 2.9`    |        x |                                             |
//! | `tmux_2_9a`   | `tmux 2.9a`   |        x |                                             |
//! | `tmux_3_0`    | `tmux 3.0`    |        x |                                             |
//! | `tmux_3_0a`   | `tmux 3.0a`   |        x | Debian Bullseye                             |
//! | `tmux_3_1`    | `tmux 3.1`    |        x | Debian experimental                         |
//! | `tmux_3_1a`   | `tmux 3.1a`   |        x |                                             |
//! | `tmux_3_1b`   | `tmux 3.1b`   |        x |                                             |
//! | `tmux_3_1c`   | `tmux 3.1c`   |        x |                                             |
//! | `tmux_3_2`    | `tmux 3.2`    |        x |                                             |
//! | `tmux_3_2a`   | `tmux 3.2a`   |        x |                                             |
//! | `tmux_3_3`    | `tmux 3.3`    |        x |                                             |
//! | `tmux_3_3a`   | `tmux 3.3a`   |        x |                                             |
//! | `tmux_X_X`    |               |        x | tmux: `main` branch; library: `dev` branch  |
//! |               |               |          |                                             |
//! | `tmux_stable` | `tmux 3.3`    |          |                                             |
//! | `tmux_latest` | `tmux 3.3a`   |          |                                             |
//!
//!
//! ```text
//! [dependencies]
//! tmux_interface = {
//!  version = "^0.1.0",
//!  features = ["tmux_2_6"]
//! }
//! ```
//!
//! By default `tmux_stable` is used. It can be removed with `--no-default-features`
//! cargo command line option or with `default-features = false` option in `Cargo.toml`
//!  You can also add
//! `features` to your dependencies entry in `Cargo.toml`, if you want to
//! specify the version of tmux you want to use.
//!
//! ```text
//! [dependencies]
//! tmux_interface = {
//!  version = "^0.1.0",
//!  default-features = false,
//!  features = ["tmux_2_6"]
//! }
//! ```
//!
//! ## 3.2. Tmux Command Alias
//!
//! `cmd_alias` use alias instead of full tmux command name (e.g. `list-sessions` -> `ls`). Enabled by default.
//!
//! ## 3.3. Repository
//!
//! ### 3.3.1. Using Crates Repository
//!
//! ```text
//! [dependencies]
//! tmux_interface = {
//!  version = "0.0.7",
//! }
//! ```
//!
//! ### 3.3.2. Using Local Repository
//!
//! ```text
//! [dependencies]
//! tmux_interface = {
//!  version = "0.0.7",
//!  path = "../tmux-interface"
//! }
//! ```
//!
//! ### 3.3.3. Using Remote Repository
//!
//! ```text
//! tmux_interface = {
//!  git = "https://github.com/AntonGepting/tmux-interface-rs.git",
//!  branch = "dev"
//! }
//! ```
//!
//! # 4. Modules Overview
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
//!     * ...
//!
//!
//! * Modes
//!     * Default Mode
//!     * Control Mode ([`control_mode`])
//!         * (unimplemented, draft)
//!     * Copy Mode
//!         * (unimplemented, draft)
//!     * Command Mode
//!         * (unimplemented)
//!     * Clock Mode
//!         * (unimplemented)
//!
//! * Formats ([`formats`])
//!     * [`Formats`][crate::formats::Formats]
//!     * [`FormatsOutput`][crate::formats::FormatsOutput]
//!     * [`Variable`][crate::formats::Variable]
//!     * [`VariableOutput`][crate::formats::VariableOutput]
//!     * ...
//!
//! * Options ([`options`])
//!
//! * Styles ([`styles`])
//!     * [`StyleList`][crate::styles::StyleList]
//!     * [`Style`][crate::styles::Style]
//!     * ...
//!
//! * Target ([`target`])
//!     * [`TargetSession`]
//!     * [`TargetWindow`]
//!     * [`TargetPane`]
//!     * ...
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
//! * Error ([`Error`])
//!
//! * ...
//!
//!
//! Main structure is [`TmuxCommand`](crate::TmuxCommand) which has all these wrapper functions implementations.
//! This goal can be reached by splitting it into two separate tasks:
//!
//! 1. Providing wrapper functions for tmux subcommands (which is sending data). Wrapper functions are
//! structured like in tmux manual in few next categories:
//!
//! 2. Parsing functions for tmux output as rust structures (which is receiving data). Parsing function are
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
//! let session_name = "example_2";
//!
//! // tmux new -d -n example_2 ; attach -t example_2 ; kill-session -t example_2
//! let output = Tmux::new()
//!                 .add_command(NewSession::new().detached().session_name(session_name))
//!                 .add_command(AttachSession::new().target_session(session_name))
//!                 .add_command(KillSession::new().target_session(session_name))
//!                 .output()
//!                 .unwrap();
//! ```
//!
//! # Examples
//!
//! Parsing examples:
//! ```
//! use crate::tmux_interface::{SessionsCtl, WindowsCtl, PanesCtl};
//!
//! let sessions = SessionsCtl::new().get_all().unwrap();
//! let windows = WindowsCtl::new().get_all().unwrap();
//! let panes = PanesCtl::new().get_all().unwrap();
//! ```
//!
//!
//! # Examples
//!
//! Change tmux command line flags, options
//! ```
//! use crate::tmux_interface::{TmuxCommand, Tmux, NewSession, KillSession};
//!
//! let mut tmux = Tmux::new()
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
//! // use crate::tmux_interface::{Tmux, NewSession, KillSession};
//!
//! // let mut tmux = Tmux::new();
//! // tmux.command(NewSession::new().detached().session_name("new_session_default")).output();
//! // tmux.command(KillSession::new().target_session("new_session_default")).output();
//! ```
//!
//! using `std::default::Default` trait:
//! ```
//! //use crate::tmux_interface::{TmuxCommand, Tmux, NewSession, TargetSession};
//!
//! //let mut tmux = Tmux::new();
//! //tmux.new_session().detached().session_name("new_session_default").output();
//! //tmux.kill_session().target_session("new_session_default").output();
//! ```
//!
//! using direct structure modification:
//! ```
//! // use crate::tmux_interface::{TmuxCommand, Tmux, NewSession, TargetSession};
//!
//! // let mut tmux = Tmux::new();
//! // let mut new_session = NewSession::new();
//! // new_session.detached();
//! // new_session.session_name("new_session_direct");
//! // new_session.output();
//! // tmux.kill_session().target_session("new_session_direct").output();
//! ```
//!
//!
//!
//! # Hierarchy
//!
//!
//! ```text
//!
//! 5. Tmux Objects Controller
//!  +---------+     +-----------+
//!  | Options |     | Variables |
//!  +---------+     +-----------+
//!  ...
//!
//! 4. Tmux Objects Getter/Setter
//!  +-----------------+
//!  | GetServerOption |
//!  +-----------------+
//!  ...
//!
//! 3. Command Builder
//!  +------+     +------------+      +---------------+        +-----+
//!  | Tmux |     | NewSession |      | AttachSession |        | ... |
//!  +------+     +------------+      +---------------+        +-----+
//!
//! 2. Tmux Command
//!  +-------------+                                  +------------+
//!  | TmuxCommand |                                  | TmuxOutput |
//!  +-------------+                                  +------------+
//!  +-----------------+
//!  | TmuxCommandList |
//!  +-----------------+
//!
//! 1. Standard Library
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
//! # Levels
//!
//! Tmux command invocation can be described and accessed on different levels:
//!
//! * 0. OS (`fork()`, `CreateProcess`, ... )
//! * 1. Standard Library ([`std::process::Command`])
//! * 2. Tmux Interface Library - Command Builder ([`TmuxCommand`], [`TmuxCommandList`])
//! * 3. Tmux Interface Library - Tmux Command Builder ([`Tmux`], [`NewSession`], [`AttachSession`] ... )
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
//!     //use cmd_builder::Cmd;
//!
//!     //let output = Cmd::new()
//!     //               .name("tmux")
//!     //               .push_flag_short('2')
//!     //               .push_flag_short('u')
//!     //               .push_flag_short('v')
//!     //               .push_cmd(
//!     //                 Cmd::new()
//!     //                   .name("new-session")
//!     //                   .push_flag_short('D')
//!     //                   .push_flag_short('E')
//!     //                   .arg("-s", "my_session").to_owned())
//!     //               .combine_short_flags()
//!     //               .to_command()
//!     //               .output()
//!     //               .unwrap();
//!     ```
//!     ```text
//!     tmux -2uv new-session -ADX -s my_session
//!     ```
//!
//! * 3. [`TmuxCommand`], [`TmuxCommandList`] - just a wrapper around [`cmd_builder::Cmd`] inside `tmux_interface` crate,
//! same functionality
//!
//!     ### Examples
//!     ```
//!     // use tmux_interface::TmuxCommand;
//!
//!     // let output = TmuxCommand::new("tmux")
//!     //                .push_flag_short('2')
//!     //                .push_flag_short('u')
//!     //                .push_flag_short('v')
//!     //                .push_cmd(TmuxCommand::new("new-session")
//!     //                  .alias("new")
//!     //                  .push_flag_short('A')
//!     //                  .push_flag_short('D')
//!     //                  .push_flag_short('E')
//!     //                  .push_option("-s", "name"))
//!     //                .output()
//!     //                .unwrap();
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
//!     * using macros
//!
//!     ### Examples
//!     ```
//!     use tmux_interface::{Tmux, KillSession, NewSession};
//!
//!     let output = Tmux::with_command(
//!                      NewSession::new()
//!                         .attach()
//!                         .detached()
//!                         .window_name("my_window_name")
//!                         .session_name("name"))
//!                    .colours256()
//!                    .force_utf8()
//!                    .verbose_logging()
//!                    .output()
//!                    .unwrap();
//!     Tmux::with_command(KillSession::new().target_session("name")).output().unwrap();
//!     ```
//!     or using macros
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
