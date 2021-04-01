#![forbid(unsafe_code)]
//#![warn(missing_docs)]

//! `tmux_interface` is a library for communication with [TMUX](https://github.com/tmux/tmux) via
//! CLI.
//!
//! # Description
//!
//! Main purpose of the `tmux_interface` library is to implement simple sending and recieving data
//! mechanisms for intercommunication with `TMUX` only via standard streams (`stdin`, `stdout`,
//! `stderr`).
//!
//! # Project Structure
//!
//! This goal can be reached by splitting it into two separate tasks:
//!
//! 1. Providing wrapper functions for tmux subcommands (which is sending data). Wrapper functions are
//! structured like in tmux manual in few next categories:
//!
//!     - Clients and Sessions ([`clients_and_sessions`](crate::commands::clients_and_sessions))
//!     - Windows and Panes ([`windows_and_panes`](crate::commands::windows_and_panes))
//!     - Key Bindings ([`key_bindings`](crate::commands::key_bindings))
//!     - Options ([`options`](crate::commands::options))
//!     - Hooks ([`hooks`](crate::commands::hooks))
//!     - Global and Session Environment ([`global_and_session_environment`](crate::commands::global_and_session_environment))
//!     - Status Line ([`status_line`](crate::commands::status_line))
//!     - Buffers ([`buffers`](crate::commands::buffers))
//!     - Miscellaneous ([`miscellaneous`](crate::commands::miscellaneous))
//!
//! Main structure is [`TmuxCommand`](crate::TmuxCommand) wich has all these wrapper functions implementations.
//!
//! 2. Parsing functions for tmux output as rust structures (which is recieving data). Parsing function are
//! structured by objects they operate with:
//!
//!     - [`Sessions`](crate::Sessions)
//!     - [`Session`](crate::Session)
//!     - [`Windows`](crate::Windows)
//!     - [`Window`](crate::Window)
//!     - [`Panes`](crate::Panes)
//!     - [`Pane`](crate::Pane)
//!     - ...
//!
//! # Conventions
//!
//! Library Functions:
//!     - Function names and their grouping are inherited from tmux manual
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::{AttachSession, NewSession, KillSession, TmuxCommand};
//!
//! // 1. TmuxCommand
//! let tmux = TmuxCommand::new();
//!
//! tmux.new_session().detached().session_name("new_session_name1").output().unwrap();
//! tmux.attach_session().target_session("new_session_name1").output().unwrap();
//! tmux.kill_session().target_session("new_session_name1").output().unwrap();
//!
//! // 2.a. One-liner
//! NewSession::new().detached().session_name("new_session_name2a").output().unwrap();
//! AttachSession::new().target_session("new_session_name2a").output().unwrap();
//! KillSession::new().target_session("new_session_name2a").output().unwrap();
//!
//! // 2.b. Multi-Liner
//! let mut new_session = NewSession::new();
//! new_session.detached();
//! new_session.session_name("new_session_name2b");
//! new_session.output().unwrap();
//!
//! let mut attach_session = AttachSession::new();
//! attach_session.target_session("new_session_name2b");
//! attach_session.output().unwrap();
//!
//! let mut kill_session = KillSession::new();
//! kill_session.target_session("new_session_name2b").output().unwrap();
//! ```
//!
//!
//! # Examples
//!
//! Parsing examples:
//! ```
//! use crate::tmux_interface::{Sessions, Session, Windows, Window, Pane, Panes, TargetSession,
//! TargetWindowExt};
//! use crate::tmux_interface::variables::session::session::SESSION_ALL;
//! use crate::tmux_interface::variables::window::window::WINDOW_ALL;
//! use crate::tmux_interface::variables::pane::pane::PANE_ALL;
//!
//! let sessions = Sessions::get(SESSION_ALL).unwrap();
//! let windows = Windows::get(&TargetSession::Raw("0"), WINDOW_ALL).unwrap();
//! let panes = Panes::get(&TargetWindowExt::raw("0:1"), PANE_ALL).unwrap();
//! ```
//!
//!
//! # Examples
//!
//! Change tmux command line flags, options
//! ```
//! use crate::tmux_interface::{TmuxCommand, NewSession, KillSession};
//!
//! let mut tmux = TmuxCommand::new();
//! tmux.bin("tmux");
//! NewSession::from(&tmux).detached().session_name("new_session_name3").output();
//! KillSession::from(&tmux).target_session("new_session_name3").output();
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
//! let mut tmux = TmuxCommand::new();
//! tmux.new_session().detached().session_name("new_session_default").output();
//! tmux.kill_session().target_session("new_session_default").output();
//! ```
//!
//! using `std::default::Default` trait:
//! ```
//! use crate::tmux_interface::{TmuxCommand, NewSession, TargetSession};
//!
//! let mut tmux = TmuxCommand::new();
//! let new_session =
//! tmux.new_session().detached().session_name("new_session_default").output();
//! tmux.kill_session().target_session("new_session_default").output();
//! ```
//!
//! using direct structure modification:
//! ```
//! use crate::tmux_interface::{TmuxCommand, NewSession, TargetSession};
//!
//! let mut tmux = TmuxCommand::new();
//! let mut new_session = NewSession::new();
//! new_session.detached();
//! new_session.session_name("new_session_direct");
//! new_session.output();
//! tmux.kill_session().target_session("new_session_direct").output();
//! ```
//!
//!
//!
//! ## Usage
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
//!     let mut tmux = TmuxCommand::new();
//!     let new_session = NewSession::new().detached().session_name("session_name").output();
//!     let attach_session = AttachSession::new().target_session(&target_session).output();
//!     tmux.send_keys().key("exit").key("C-m").output();
//!     tmux.kill_session().target_session(&target_session).output();
//!     ```
//!
//!

pub mod error;

pub mod commands;
pub mod options;
pub mod target;
#[cfg(feature = "tmux_1_6")]
pub mod variables;

pub mod version;

// common options
pub use crate::options::StatusKeys;
pub use crate::options::Switch;

// server options
#[cfg(feature = "tmux_1_2")]
pub use crate::options::server_options::ServerOptions;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::server_options::ServerOptionsBuilder;
// structures & enums
#[cfg(feature = "tmux_1_5")]
pub use crate::options::server_options::SetClipboard;
// constants
// TODO: sort alphabetically
#[cfg(feature = "tmux_3_1")]
pub use crate::options::server_options::BACKSPACE;
#[cfg(feature = "tmux_1_5")]
pub use crate::options::server_options::BUFFER_LIMIT;
#[cfg(feature = "tmux_2_4")]
pub use crate::options::server_options::COMMAND_ALIAS;
#[cfg(feature = "tmux_2_1")]
pub use crate::options::server_options::DEFAULT_TERMINAL;
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
pub use crate::options::server_options::DETACH_ON_DESTROY;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::server_options::ESCAPE_TIME;
#[cfg(feature = "tmux_2_7")]
pub use crate::options::server_options::EXIT_EMPTY;
#[cfg(feature = "tmux_1_4")]
pub use crate::options::server_options::EXIT_UNATTACHED;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::server_options::FOCUS_EVENTS;
#[cfg(feature = "tmux_2_1")]
pub use crate::options::server_options::HISTORY_FILE;
#[cfg(feature = "tmux_2_0")]
pub use crate::options::server_options::MESSAGE_LIMIT;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub use crate::options::server_options::QUIET;
#[cfg(feature = "tmux_1_5")]
pub use crate::options::server_options::SET_CLIPBOARD;
#[cfg(feature = "tmux_2_0")]
pub use crate::options::server_options::TERMINAL_OVERRIDES;
#[cfg(feature = "tmux_3_0")]
pub use crate::options::server_options::USER_KEYS;
// all & none
#[cfg(feature = "tmux_1_0")]
pub use crate::options::server_options::SERVER_OPTIONS_ALL;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::server_options::SERVER_OPTIONS_NONE;

// session otions
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::SessionOptions;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::SessionOptionsBuilder;
// structures & enums
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::Action;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::Activity;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::Status;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::StatusJustify;
#[cfg(feature = "tmux_1_7")]
pub use crate::options::session_options::StatusPosition;
// constants
// TODO: sort alphabetically
#[cfg(feature = "tmux_2_6")]
pub use crate::options::session_options::ACTIVITY_ACTION;
#[cfg(feature = "tmux_1_8")]
pub use crate::options::session_options::ASSUME_PASTE_TIME;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::BASE_INDEX;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::BELL_ACTION;
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
pub use crate::options::session_options::BELL_ON_ALERT;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
pub use crate::options::session_options::BUFFER_LIMIT;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::DEFAULT_COMMAND;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::DEFAULT_PATH;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::DEFAULT_SHELL;
#[cfg(feature = "tmux_2_9")]
pub use crate::options::session_options::DEFAULT_SIZE;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub use crate::options::session_options::DEFAULT_TERMINAL;
#[cfg(feature = "tmux_1_4")]
pub use crate::options::session_options::DESTROY_UNATTACHED;
#[cfg(feature = "tmux_1_4")]
pub use crate::options::session_options::DETACH_ON_DESTROY;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::session_options::DISPLAY_PANES_ACTIVE_COLOUR;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::DISPLAY_PANES_COLOUR;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::DISPLAY_PANES_TIME;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::DISPLAY_TIME;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::HISTORY_LIMIT;
#[cfg(feature = "tmux_2_2")]
pub use crate::options::session_options::KEY_TABLE;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::LOCK_AFTER_TIME;
#[cfg(feature = "tmux_1_1")]
pub use crate::options::session_options::LOCK_COMMAND;
#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
pub use crate::options::session_options::LOCK_SERVER;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::MESSAGE_ATTR;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::MESSAGE_BG;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::MESSAGE_COMMAND_ATTR;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::MESSAGE_COMMAND_BG;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::MESSAGE_COMMAND_FG;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::session_options::MESSAGE_COMMAND_STYLE;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::MESSAGE_FG;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub use crate::options::session_options::MESSAGE_LIMIT;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::session_options::MESSAGE_STYLE;
#[cfg(feature = "tmux_2_1")]
pub use crate::options::session_options::MOUSE;
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub use crate::options::session_options::MOUSE_RESIZE_PANE;
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub use crate::options::session_options::MOUSE_SELECT_PANE;
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub use crate::options::session_options::MOUSE_SELECT_WINDOW;
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
pub use crate::options::session_options::MOUSE_UTF8;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::PANE_ACTIVE_BORDER_BG;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::PANE_ACTIVE_BORDER_FG;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::PANE_ACTIVE_BORDER_STYLE;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_0")))]
pub use crate::options::session_options::PANE_BORDER_BG;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::PANE_BORDER_FG;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub use crate::options::session_options::PANE_BORDER_STYLE;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::PREFIX;
#[cfg(feature = "tmux_1_6")]
pub use crate::options::session_options::PREFIX2;
#[cfg(feature = "tmux_1_7")]
pub use crate::options::session_options::RENUMBER_WINDOWS;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::REPEAT_TIME;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
pub use crate::options::session_options::SET_REMAIN_ON_EXIT;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::SET_TITLES;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::SET_TITLES_STRING;
#[cfg(feature = "tmux_2_6")]
pub use crate::options::session_options::SILENCE_ACTION;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::STATUS;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_ATTR;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_BG;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_FG;
#[cfg(feature = "tmux_2_9")]
pub use crate::options::session_options::STATUS_FORMAT;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::STATUS_INTERVAL;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::STATUS_JUSTIFY;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::STATUS_KEYS;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::STATUS_LEFT;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_LEFT_ATTR;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_LEFT_BG;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_LEFT_FG;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::STATUS_LEFT_LENGTH;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::session_options::STATUS_LEFT_STYLE;
#[cfg(feature = "tmux_1_7")]
pub use crate::options::session_options::STATUS_POSITION;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::STATUS_RIGHT;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_RIGHT_ATTR;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_RIGHT_BG;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::session_options::STATUS_RIGHT_FG;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::STATUS_RIGHT_LENGTH;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::session_options::STATUS_RIGHT_STYLE;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::session_options::STATUS_STYLE;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
pub use crate::options::session_options::STATUS_UTF8;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub use crate::options::session_options::TERMINAL_OVERRIDES;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::UPDATE_ENVIRONMENT;
#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
pub use crate::options::session_options::USER_KEYS;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::VISUAL_ACTIVITY;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::VISUAL_BELL;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub use crate::options::session_options::VISUAL_CONTENT;
#[cfg(feature = "tmux_1_4")]
pub use crate::options::session_options::VISUAL_SILENCE;
#[cfg(feature = "tmux_1_6")]
pub use crate::options::session_options::WORD_SEPARATORS;
// all
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::SESSION_OPTIONS_ALL;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session_options::SESSION_OPTIONS_NONE;

// window options
#[cfg(feature = "tmux_1_2")]
pub use crate::options::window_options::WindowOptions;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::window_options::WindowOptionsBuilder;
// structures & enums
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::ClockModeStyle;
#[cfg(feature = "tmux_2_3")]
pub use crate::options::window_options::PaneBorderStatus;
#[cfg(feature = "tmux_2_9")]
pub use crate::options::window_options::WindowSize;
// constants
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::AGGRESIVE_RESIZE;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
pub use crate::options::window_options::ALLOW_RENAME;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
pub use crate::options::window_options::ALTERNAME_SCREEN;
#[cfg(feature = "tmux_1_0")] // 0.8
pub use crate::options::window_options::AUTOMATIC_RENAME;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::window_options::AUTOMATIC_RENAME_FORMAT;
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub use crate::options::window_options::C0_CHANGE_INTERVAL;
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub use crate::options::window_options::C0_CHANGE_TRIGGER;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::CLOCK_MODE_COLOUR;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::CLOCK_MODE_STYLE;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub use crate::options::window_options::FORCE_HEIGHT;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub use crate::options::window_options::FORCE_WIDTH;
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub use crate::options::window_options::LAYOUT_HISTORY_LIMIT;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::MAIN_PANE_HEIGHT;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::MAIN_PANE_WIDTH;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::MODE_ATTR;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::MODE_BG;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::MODE_FG;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::MODE_KEYS;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub use crate::options::window_options::MODE_MOUSE;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::window_options::MODE_STYLE;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::MONITOR_ACTIVITY;
#[cfg(feature = "tmux_2_6")]
pub use crate::options::window_options::MONITOR_BELL;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub use crate::options::window_options::MONITOR_CONTENT;
#[cfg(feature = "tmux_1_4")]
pub use crate::options::window_options::MONITOR_SILENCE;
#[cfg(feature = "tmux_1_4")]
pub use crate::options::window_options::OTHER_PANE_HEIGHT;
#[cfg(feature = "tmux_1_4")]
pub use crate::options::window_options::OTHER_PANE_WIDTH;
#[cfg(feature = "tmux_2_0")]
pub use crate::options::window_options::PANE_ACTIVE_BORDER_STYLE;
#[cfg(feature = "tmux_1_6")]
pub use crate::options::window_options::PANE_BASE_INDEX;
#[cfg(feature = "tmux_2_3")]
pub use crate::options::window_options::PANE_BORDER_FORMAT;
#[cfg(feature = "tmux_2_3")]
pub use crate::options::window_options::PANE_BORDER_STATUS;
#[cfg(feature = "tmux_2_0")]
pub use crate::options::window_options::PANE_BORDER_STYLE;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
pub use crate::options::window_options::REMAIN_ON_EXIT;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::SYNCHRONIZE_PANES;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
pub use crate::options::window_options::UTF8;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub use crate::options::window_options::WINDOW_ACTIVE_STYLE;
#[cfg(feature = "tmux_2_9")]
pub use crate::options::window_options::WINDOW_SIZE;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_ACTIVITY_ATTR;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_ACTIVITY_BG;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_ACTIVITY_FG;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::window_options::WINDOW_STATUS_ACTIVITY_STYLE;
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub use crate::options::window_options::WINDOW_STATUS_ALERT_ATTR;
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub use crate::options::window_options::WINDOW_STATUS_ALERT_BG;
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub use crate::options::window_options::WINDOW_STATUS_ALERT_FG;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_ATTR;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_BELL_ATTR;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_BELL_BG;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_BELL_FG;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::window_options::WINDOW_STATUS_BELL_STYLE;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_BG;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_CONTENT_ATTR;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_CONTENT_BG;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_CONTENT_FG;
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub use crate::options::window_options::WINDOW_STATUS_CONTENT_STYLE;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_CURRENT_ATTR;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_CURRENT_BG;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_CURRENT_FG;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::window_options::WINDOW_STATUS_CURRENT_FORMAT;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::window_options::WINDOW_STATUS_CURRENT_STYLE;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_FG;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::window_options::WINDOW_STATUS_FORMAT;
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_LAST_ATTR;
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_LAST_BG;
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub use crate::options::window_options::WINDOW_STATUS_LAST_FG;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::window_options::WINDOW_STATUS_LAST_STYLE;
#[cfg(feature = "tmux_1_7")]
pub use crate::options::window_options::WINDOW_STATUS_SEPARATOR;
#[cfg(feature = "tmux_1_9")]
pub use crate::options::window_options::WINDOW_STATUS_STYLE;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub use crate::options::window_options::WINDOW_STYLE;
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
pub use crate::options::window_options::WORD_SEPARATORS;
#[cfg(feature = "tmux_1_7")]
pub use crate::options::window_options::WRAP_SEARCH;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::XTERM_KEYS;
// all & none
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::WINDOW_OPTIONS_ALL;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::window_options::WINDOW_OPTIONS_NONE;

// pane options
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::PaneOptions;
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::PaneOptionsBuilder;
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::ALLOW_RENAME;
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::ALTERNATE_SCREEN;
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::PANE_OPTIONS;
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::REMAIN_ON_EXIT;
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::WINDOW_ACTIVE_STYLE;
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::WINDOW_STYLE;
// all & none
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::PANE_OPTIONS_ALL;
#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane_options::PANE_OPTIONS_NONE;

pub use self::commands::tmux_command::TmuxCommand;
pub use self::commands::tmux_output::TmuxOutput;

// buffers
#[cfg(feature = "tmux_1_3")]
pub use self::commands::buffers::choose_buffer::ChooseBuffer;
#[cfg(feature = "tmux_0_9")]
pub use self::commands::buffers::clear_history::ClearHistory;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::buffers::delete_buffer::DeleteBuffer;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::buffers::list_buffers::ListBuffers;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::buffers::load_buffer::LoadBuffer;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::buffers::paste_buffer::PasteBuffer;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::buffers::save_buffer::SaveBuffer;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::buffers::set_buffer::SetBuffer;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::buffers::show_buffer::ShowBuffer;

// clients and sessions
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::attach_session::AttachSession;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::detach_client::DetachClient;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::has_session::HasSession;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::kill_server::KillServer;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::kill_session::KillSession;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::list_clients::ListClients;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::list_commands::ListCommands;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::list_sessions::ListSessions;
#[cfg(feature = "tmux_1_1")]
pub use self::commands::clients_and_sessions::lock_client::LockClient;
#[cfg(feature = "tmux_1_1")]
pub use self::commands::clients_and_sessions::lock_session::LockSession;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::new_session::NewSession;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::refresh_client::RefreshClient;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::rename_session::RenameSession;
#[cfg(feature = "tmux_1_2")]
pub use self::commands::clients_and_sessions::show_messages::ShowMessages;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::source_file::SourceFile;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::start_server::StartServer;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::suspend_client::SuspendClient;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::clients_and_sessions::switch_client::SwitchClient;

// global and session environment
#[cfg(feature = "tmux_1_0")]
pub use self::commands::global_and_session_environment::set_environment::SetEnvironment;
#[cfg(feature = "tmux_1_0")]
pub use self::commands::global_and_session_environment::show_environment::ShowEnvironment;

// hooks
#[cfg(feature = "tmux_2_2")]
pub use self::commands::hooks::set_hook::SetHook;
#[cfg(feature = "tmux_2_2")]
pub use self::commands::hooks::show_hooks::ShowHooks;

// key bindings
#[cfg(feature = "tmux_0_8")]
pub use self::commands::key_bindings::bind_key::BindKey;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::key_bindings::list_keys::ListKeys;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::key_bindings::send_keys::SendKeys;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::key_bindings::send_prefix::SendPrefix;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::key_bindings::unbind_key::UnbindKey;

// miscellaneous
#[cfg(feature = "tmux_0_8")]
pub use self::commands::miscellaneous::clock_mode::ClockMode;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::miscellaneous::if_shell::IfShell;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::miscellaneous::lock_server::LockServer;
#[cfg(feature = "tmux_1_1")]
pub use self::commands::miscellaneous::run_shell::RunShell;
#[cfg(feature = "tmux_1_8")]
pub use self::commands::miscellaneous::wait_for::WaitFor;

// options
#[cfg(feature = "tmux_0_8")]
pub use self::commands::options::set_option::SetOption;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::options::set_window_option::SetWindowOption;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::options::show_options::ShowOptions;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::options::show_window_options::ShowWindowOptions;

// status line
#[cfg(feature = "tmux_0_8")]
pub use self::commands::status_line::command_prompt::CommandPrompt;
#[cfg(feature = "tmux_0_9")]
pub use self::commands::status_line::confirm_before::ConfirmBefore;
#[cfg(feature = "tmux_3_0")]
pub use self::commands::status_line::display_menu::DisplayMenu;
#[cfg(feature = "tmux_1_0")]
pub use self::commands::status_line::display_message::DisplayMessage;

// windows and panes
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::break_pane::BreakPane;
#[cfg(feature = "tmux_1_2")]
pub use self::commands::windows_and_panes::capture_pane::CapturePane;
#[cfg(feature = "tmux_1_0")]
pub use self::commands::windows_and_panes::choose_client::ChooseClient;
#[cfg(feature = "tmux_1_7")]
pub use self::commands::windows_and_panes::choose_tree::ChooseTree;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::copy_mode::CopyMode;
#[cfg(feature = "tmux_1_0")]
pub use self::commands::windows_and_panes::display_panes::DisplayPanes;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::find_window::FindWindow;
#[cfg(feature = "tmux_1_2")]
pub use self::commands::windows_and_panes::join_pane::JoinPane;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::kill_pane::KillPane;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::kill_window::KillWindow;
#[cfg(feature = "tmux_1_4")]
pub use self::commands::windows_and_panes::last_pane::LastPane;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::last_window::LastWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::link_window::LinkWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::list_panes::ListPanes;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::list_windows::ListWindows;
#[cfg(feature = "tmux_1_7")]
pub use self::commands::windows_and_panes::move_pane::MovePane;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::move_window::MoveWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::new_window::NewWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::next_layout::NextLayout;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::next_window::NextWindow;
#[cfg(feature = "tmux_1_1")]
pub use self::commands::windows_and_panes::pipe_pane::PipePane;
#[cfg(feature = "tmux_1_3")]
pub use self::commands::windows_and_panes::previous_layout::PreviousLayout;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::previous_window::PreviousWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::rename_window::RenameWindow;
#[cfg(feature = "tmux_0_9")]
pub use self::commands::windows_and_panes::resize_pane::ResizePane;
#[cfg(feature = "tmux_2_9")]
pub use self::commands::windows_and_panes::resize_window::ResizeWindow;
#[cfg(feature = "tmux_1_5")]
pub use self::commands::windows_and_panes::respawn_pane::RespawnPane;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::respawn_window::RespawnWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::rotate_window::RotateWindow;
#[cfg(feature = "tmux_0_9")]
pub use self::commands::windows_and_panes::select_layout::SelectLayot;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::select_pane::SelectPane;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::select_window::SelectWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::split_window::SplitWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::swap_pane::SwapPane;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::swap_window::SwapWindow;
#[cfg(feature = "tmux_0_8")]
pub use self::commands::windows_and_panes::unlink_window::UnlinkWindow;

// enums
pub use crate::commands::PaneSize;

pub use crate::target::target_pane::{TargetPane, TargetPaneExt, TargetPaneToken};
pub use crate::target::target_session::TargetSession;
pub use crate::target::target_window::{TargetWindow, TargetWindowExt, TargetWindowToken};

// enums
#[cfg(feature = "tmux_1_6")]
pub use self::variables::layout::layout_cell::LayoutType;
// structs
#[cfg(feature = "tmux_1_6")]
pub use self::variables::layout::layout::Layout;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::layout::layout_cell::LayoutCell;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::layout::layout_checksum::LayoutChecksum;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::pane::pane::Pane;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::pane::pane_tabs::PaneTabs;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::pane::panes::Panes;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::session::session::Session;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::session::session_stack::SessionStack;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::session::sessions::Sessions;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::window::window::Window;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::window::window_flag::WindowFlag;
#[cfg(feature = "tmux_1_6")]
pub use self::variables::window::windows::Windows;

pub use self::version::Version;

// structs
pub use self::error::Error;

//mod options_tests;
//mod tmux_option_tests;
mod version_tests;

// consts
// TODO: add all
#[cfg(feature = "tmux_2_1")]
pub use crate::variables::session::session::SESSION_ACTIVITY;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
pub use crate::variables::session::session::SESSION_ACTIVITY_STRING;
#[cfg(feature = "tmux_2_1")]
pub use crate::variables::session::session::SESSION_ALERTS;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::session::session::SESSION_ATTACHED;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::session::session::SESSION_ATTACHED_LIST;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::session::session::SESSION_CREATED;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
pub use crate::variables::session::session::SESSION_CREATED_STRING;
#[cfg(feature = "tmux_2_6")]
pub use crate::variables::session::session::SESSION_FORMAT;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::session::session::SESSION_GROUP;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::session::session::SESSION_GROUPED;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::session::session::SESSION_GROUP_ATTACHED;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::session::session::SESSION_GROUP_ATTACHED_LIST;
#[cfg(feature = "tmux_2_7")]
pub use crate::variables::session::session::SESSION_GROUP_LIST;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::session::session::SESSION_GROUP_MANY_ATTACHED;
#[cfg(feature = "tmux_2_7")]
pub use crate::variables::session::session::SESSION_GROUP_SIZE;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
pub use crate::variables::session::session::SESSION_HEIGHT;
#[cfg(feature = "tmux_1_8")]
pub use crate::variables::session::session::SESSION_ID;
#[cfg(feature = "tmux_2_1")]
pub use crate::variables::session::session::SESSION_LAST_ATTACHED;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
pub use crate::variables::session::session::SESSION_LAST_ATTACHED_STRING;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::session::session::SESSION_MANY_ATTACHED;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::session::session::SESSION_NAME;
#[cfg(feature = "tmux_2_5")]
pub use crate::variables::session::session::SESSION_STACK;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
pub use crate::variables::session::session::SESSION_WIDTH;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::session::session::SESSION_WINDOWS;

#[cfg(feature = "tmux_1_6")]
pub use crate::variables::session::session::SESSION_NONE;
//pub use crate::variables::session::session::SESSION_DEFAULT;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::session::session::SESSION_ALL;

#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_ACTIVE;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_ACTIVE_CLIENTS;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_ACTIVE_CLIENTS_LIST;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_ACTIVE_SESSIONS;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_ACTIVE_SESSIONS_LIST;
#[cfg(feature = "tmux_2_1")]
pub use crate::variables::window::window::WINDOW_ACTIVITY;
#[cfg(any(
    all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
    feature = "tmux_2_3"
))]
pub use crate::variables::window::window::WINDOW_ACTIVITY_FLAG;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
pub use crate::variables::window::window::WINDOW_ACTIVITY_STRING;
#[cfg(feature = "tmux_1_9")]
pub use crate::variables::window::window::WINDOW_BELL_FLAG;
#[cfg(feature = "tmux_2_9")]
pub use crate::variables::window::window::WINDOW_BIGGER;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_CELL_HEIGHT;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_CELL_WIDTH;
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub use crate::variables::window::window::WINDOW_CONTENT_FLAG;
#[cfg(feature = "tmux_2_9")]
pub use crate::variables::window::window::WINDOW_END_FLAG;
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
pub use crate::variables::window::window::WINDOW_FIND_MATCHES;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_FLAGS;
#[cfg(feature = "tmux_2_6")]
pub use crate::variables::window::window::WINDOW_FORMAT;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_HEIGHT;
#[cfg(feature = "tmux_1_7")]
pub use crate::variables::window::window::WINDOW_ID;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_INDEX;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::window::window::WINDOW_LAST_FLAG;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_LAYOUT;
#[cfg(feature = "tmux_2_1")]
pub use crate::variables::window::window::WINDOW_LINKED;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_LINKED_SESSIONS;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_LINKED_SESSIONS_LIST;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::window::window::WINDOW_MARKED_FLAG;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_NAME;
#[cfg(feature = "tmux_2_9")]
pub use crate::variables::window::window::WINDOW_OFFSET_X;
#[cfg(feature = "tmux_2_9")]
pub use crate::variables::window::window::WINDOW_OFFSET_Y;
#[cfg(feature = "tmux_1_7")]
pub use crate::variables::window::window::WINDOW_PANES;
#[cfg(feature = "tmux_1_9")]
pub use crate::variables::window::window::WINDOW_SILENCE_FLAG;
#[cfg(feature = "tmux_2_5")]
pub use crate::variables::window::window::WINDOW_STACK_INDEX;
#[cfg(feature = "tmux_2_9")]
pub use crate::variables::window::window::WINDOW_START_FLAG;
#[cfg(feature = "tmux_2_2")]
pub use crate::variables::window::window::WINDOW_VISIBLE_LAYOUT;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_WIDTH;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::window::window::WINDOW_ZOOMED_FLAG;

#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_NONE;
//pub use crate::variables::window::window::WINDOW_DEFAULT;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::window::window::WINDOW_ALL;

#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_ACTIVE;
#[cfg(feature = "tmux_2_6")]
pub use crate::variables::pane::pane::PANE_AT_BOTTOM;
#[cfg(feature = "tmux_2_6")]
pub use crate::variables::pane::pane::PANE_AT_LEFT;
#[cfg(feature = "tmux_2_6")]
pub use crate::variables::pane::pane::PANE_AT_RIGHT;
#[cfg(feature = "tmux_2_6")]
pub use crate::variables::pane::pane::PANE_AT_TOP;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::pane::pane::PANE_BOTTOM;
#[cfg(feature = "tmux_1_8")]
pub use crate::variables::pane::pane::PANE_CURRENT_COMMAND;
#[cfg(feature = "tmux_1_7")]
pub use crate::variables::pane::pane::PANE_CURRENT_PATH;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_DEAD;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::pane::pane::PANE_DEAD_STATUS;
#[cfg(feature = "tmux_2_6")]
pub use crate::variables::pane::pane::PANE_FORMAT;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_HEIGHT;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_ID;
#[cfg(feature = "tmux_1_7")]
pub use crate::variables::pane::pane::PANE_INDEX;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::pane::pane::PANE_INPUT_OFF;
#[cfg(feature = "tmux_1_8")]
pub use crate::variables::pane::pane::PANE_IN_MODE;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::pane::pane::PANE_LEFT;
#[cfg(feature = "tmux_3_0")]
pub use crate::variables::pane::pane::PANE_MARKED;
#[cfg(feature = "tmux_3_0")]
pub use crate::variables::pane::pane::PANE_MARKED_SET;
#[cfg(feature = "tmux_2_5")]
pub use crate::variables::pane::pane::PANE_MODE;
#[cfg(feature = "tmux_3_1")]
pub use crate::variables::pane::pane::PANE_PATH;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_PID;
#[cfg(feature = "tmux_2_6")]
pub use crate::variables::pane::pane::PANE_PIPE;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::pane::pane::PANE_RIGHT;
#[cfg(feature = "tmux_2_5")]
pub use crate::variables::pane::pane::PANE_SEARCH_STRING;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_START_COMMMAND;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
pub use crate::variables::pane::pane::PANE_START_PATH;
#[cfg(feature = "tmux_1_9")]
pub use crate::variables::pane::pane::PANE_SYNCHRONIZED;
#[cfg(feature = "tmux_1_8")]
pub use crate::variables::pane::pane::PANE_TABS;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_TITLE;
#[cfg(feature = "tmux_2_0")]
pub use crate::variables::pane::pane::PANE_TOP;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_TTY;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_WIDTH;

#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_NONE;
//pub use crate::variables::pane::pane::PANE_DEFAULT;
#[cfg(feature = "tmux_1_6")]
pub use crate::variables::pane::pane::PANE_ALL;
