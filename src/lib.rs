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
//! 1. Providing wrapper functions for tmux subcommands (sending data). Wrapper functions are
//! structured like in tmux manual in few next categories:
//!
//!     - Clients and Sessions ([`clients_and_sessions`](crate::clients_and_sessions))
//!     - Windows and Panes ([`windows_and_panes`](crate::windows_and_panes))
//!     - Key Bindings ([`key_bindings`](crate::key_bindings))
//!     - Options ([`options`](crate::options))
//!     - Hooks ([`hooks`](crate::hooks))
//!     - Global and Session Environment ([`global_and_session_environment`](crate::global_and_session_environment))
//!     - Status Line ([`status_line`](crate::status_line))
//!     - Buffers ([`buffers`](crate::buffers))
//!     - Miscellaneous ([`miscellaneous`](crate::miscellaneous))
//!
//! Main structure is [`TmuxInterface`](crate::tmux_interface::TmuxInterface) wich has all these wrapper functions implementations.
//!
//! 2. Parsing functions for tmux output as rust structures (recieving data). Parsing function are
//! structured by objects they operate with:
//!
//!     - [`Sessions`](crate::Sessions)
//!     - [`Session`](crate::Session)
//!     - [`Windows`](crate::Windows)
//!     - [`Window`](crate::Window)
//!     - [`Panes`](crate::Panes)
//!     - [`Pane`](crate::Pane)
//!     - ...
//!     - [`TmuxOption`](crate::TmuxOption)
//!
//! # Conventions
//!
//! Library Functions:
//! 1. Function names and their grouping are inherited from tmux manual
//! 2. Function arguments and their optionality inherited from tmux manual
//! 3. Functions can have max. 4 arguments, otherwise a structure will be used
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::{TmuxInterface, AttachSession, NewSession, TargetSession};
//!
//! let mut tmux = TmuxInterface::new();
//!
//! let new_session = NewSession {
//!     detached: Some(true),
//!     session_name: Some("test_session_name1"),
//!     ..Default::default()
//! };
//! tmux.new_session(Some(&new_session)).unwrap();
//! let attach_session = AttachSession {
//!    target_session: Some(&TargetSession::Raw("test_session_name1")),
//!    ..Default::default()
//! };
//! tmux.attach_session(Some(&attach_session)).unwrap();
//! // if exists
//! tmux.kill_session(None, None, Some(&TargetSession::Raw("test_session_name1"))).unwrap();
//!
//! // or alternatively
//! let mut new_session = NewSession::new();
//! new_session.detached = Some(true);
//! new_session.session_name = Some("test_session_name2");
//! tmux.new_session(Some(&new_session)).unwrap();
//! let mut attach_session = AttachSession::new();
//! attach_session.target_session = Some(&TargetSession::Raw("test_session_name2"));
//! // if exists
//! tmux.kill_session(None, None, Some(&TargetSession::Raw("test_session_name2"))).unwrap();
//! ```
//!
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::{Sessions, Session, Windows, Window, Pane, Panes, TargetSession,
//! TargetWindowExt};
//! use crate::tmux_interface::response::session::session::SESSION_ALL;
//! use crate::tmux_interface::response::window::window::WINDOW_ALL;
//! use crate::tmux_interface::response::pane::pane::PANE_ALL;
//!
//! let sessions = Sessions::get(SESSION_ALL).unwrap();
//! let windows = Windows::get(&TargetSession::Raw("0"), WINDOW_ALL).unwrap();
//! let panes = Panes::get(&TargetWindowExt::raw("0:1"), PANE_ALL).unwrap();
//! ```
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::{TmuxInterface, NewSession, TargetSession};
//!
//! let mut tmux = TmuxInterface::new();
//! tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
//!     // changing of binary name, its arguments, subcommand and its parameters are allowed
//!     // inside callback function
//!     *bin = "tmux".to_string();
//!     // display newly set variables
//!     println!("pre hook: {:?} {:?} {:?}", bin, options, subcmd);
//!     //Err(Error::new("pre hook error"))
//!     Ok(None)
//! }));
//!
//! let new_session = NewSession {
//!     detached: Some(true),
//!     session_name: Some("test_session_name1"),
//!     ..Default::default()
//! };
//! tmux.new_session(Some(&new_session)).unwrap();
//! tmux.pre_hook = None;
//! tmux.kill_session(None, None, Some(&TargetSession::Raw("test_session_name1"))).unwrap();
//! ```
//!
//! # New session
//!
//! ## Examples
//! Create a new tmux session without any additional parameters (alternative to: `tmux new-session`)
//!
//! ```text
//! use crate::tmux_interface::TmuxInterface;
//!
//! let mut tmux = TmuxInterface::new();
//! tmux.new_session(None).unwrap();
//! ```
//!
//! ## Examples
//!
//! Create a new tmux session with some additional parameters (alternative to: `tmux new -d -s new_session`)
//! using builder pattern:
//!
//! ```
//! use crate::tmux_interface::{TmuxInterface, TargetSession, NewSessionBuilder};
//!
//! let mut tmux = TmuxInterface::new();
//! let new_session = NewSessionBuilder::new().detached().session_name("new_session_builder").build();
//! tmux.new_session(Some(&new_session)).unwrap();
//! tmux.kill_session(None, None, Some(&TargetSession::Raw("new_session_builder"))).unwrap();
//! ```
//!
//! using `std::default::Default` trait:
//! ```
//! use crate::tmux_interface::{TmuxInterface, NewSession, TargetSession};
//!
//! let mut tmux = TmuxInterface::new();
//! let new_session = NewSession {
//!     detached: Some(true),
//!     session_name: Some("new_session_default"),
//!     ..Default::default()
//! };
//! tmux.new_session(Some(&new_session)).unwrap();
//! tmux.kill_session(None, None, Some(&TargetSession::Raw("new_session_default"))).unwrap();
//! ```
//!
//! using direct structure modification:
//! ```
//! use crate::tmux_interface::{TmuxInterface, NewSession, TargetSession};
//!
//! let mut tmux = TmuxInterface::new();
//! let mut new_session = NewSession::new();
//! new_session.detached = Some(true);
//! new_session.session_name = Some("new_session_direct");
//! tmux.new_session(Some(&new_session)).unwrap();
//! tmux.kill_session(None, None, Some(&TargetSession::Raw("new_session_direct"))).unwrap();
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
//!     use tmux_interface::{AttachSession, NewSession, TargetPane, TargetSession, TmuxInterface};
//!
//!     let target_session = TargetSession::Raw("session_name");
//!     let mut tmux = TmuxInterface::new();
//!     let new_session = NewSession {
//!         detached: Some(true),
//!         session_name: Some("session_name"),
//!         ..Default::default()
//!     };
//!     tmux.new_session(Some(&new_session)).unwrap();
//!     let attach_session = AttachSession {
//!         target_session: Some(&target_session),
//!         ..Default::default()
//!     };
//!     tmux.send_keys::<TargetPane>(None, &vec!["exit", "C-m"])
//!         .unwrap();
//!     tmux.attach_session(Some(&attach_session)).unwrap();
//!     tmux.kill_session(None, None, Some(&target_session))
//!         .unwrap();
//!     ```
//!
//!

pub mod error;

pub mod common;
pub mod request;
pub mod response;

pub mod tmux_interface;
pub mod tmux_option;
pub mod version;

pub use self::tmux_interface::TmuxInterface;

// common options
pub use crate::common::StatusKeys;
pub use crate::common::Switch;

// server options
pub use crate::common::server_options::ServerOptions;
pub use crate::common::server_options::ServerOptionsBuilder;
pub use crate::common::server_options::SetClipboard;
pub use crate::common::server_options::BACKSPACE;
pub use crate::common::server_options::BUFFER_LIMIT;
pub use crate::common::server_options::COMMAND_ALIAS;
pub use crate::common::server_options::DEFAULT_TERMINAL;
pub use crate::common::server_options::ESCAPE_TIME;
pub use crate::common::server_options::EXIT_EMPTY;
pub use crate::common::server_options::EXIT_UNATTACHED;
pub use crate::common::server_options::FOCUS_EVENTS;
pub use crate::common::server_options::HISTORY_FILE;
pub use crate::common::server_options::MESSAGE_LIMIT;
pub use crate::common::server_options::SERVER_OPTIONS_ALL;
pub use crate::common::server_options::SERVER_OPTIONS_NONE;
pub use crate::common::server_options::SET_CLIPBOARD;
pub use crate::common::server_options::TERMINAL_OVERRIDES;
pub use crate::common::server_options::USER_KEYS;

// session otions
pub use crate::common::session_options::Action;
pub use crate::common::session_options::Activity;
pub use crate::common::session_options::SessionOptions;
pub use crate::common::session_options::SessionOptionsBuilder;
pub use crate::common::session_options::Status;
pub use crate::common::session_options::StatusJustify;
pub use crate::common::session_options::StatusPosition;
pub use crate::common::session_options::ACTIVITY_ACTION;
pub use crate::common::session_options::ASSUME_PASTE_TIME;
pub use crate::common::session_options::BASE_INDEX;
pub use crate::common::session_options::BELL_ACTION;
pub use crate::common::session_options::DEFAULT_COMMAND;
pub use crate::common::session_options::DEFAULT_SHELL;
pub use crate::common::session_options::DEFAULT_SIZE;
pub use crate::common::session_options::DESTROY_UNATTACHED;
pub use crate::common::session_options::DETACH_ON_DESTROY;
pub use crate::common::session_options::DISPLAY_PANES_ACTIVE_COLOUR;
pub use crate::common::session_options::DISPLAY_PANES_COLOUR;
pub use crate::common::session_options::DISPLAY_PANES_TIME;
pub use crate::common::session_options::DISPLAY_TIME;
pub use crate::common::session_options::HISTORY_LIMIT;
pub use crate::common::session_options::KEY_TABLE;
pub use crate::common::session_options::LOCK_AFTER_TIME;
pub use crate::common::session_options::LOCK_COMMAND;
pub use crate::common::session_options::MESSAGE_COMMAND_STYLE;
pub use crate::common::session_options::MESSAGE_STYLE;
pub use crate::common::session_options::MOUSE;
pub use crate::common::session_options::PREFIX;
pub use crate::common::session_options::PREFIX2;
pub use crate::common::session_options::RENUMBER_WINDOWS;
pub use crate::common::session_options::REPEAT_TIME;
pub use crate::common::session_options::SESSION_OPTIONS_ALL;
pub use crate::common::session_options::SESSION_OPTIONS_NONE;
pub use crate::common::session_options::SET_TITLES;
pub use crate::common::session_options::SET_TITLES_STRING;
pub use crate::common::session_options::SILENCE_ACTION;
pub use crate::common::session_options::STATUS;
pub use crate::common::session_options::STATUS_FORMAT;
pub use crate::common::session_options::STATUS_INTERVAL;
pub use crate::common::session_options::STATUS_JUSTIFY;
pub use crate::common::session_options::STATUS_KEYS;
pub use crate::common::session_options::STATUS_LEFT;
pub use crate::common::session_options::STATUS_LEFT_LENGTH;
pub use crate::common::session_options::STATUS_LEFT_STYLE;
pub use crate::common::session_options::STATUS_POSITION;
pub use crate::common::session_options::STATUS_RIGHT;
pub use crate::common::session_options::STATUS_RIGHT_LENGTH;
pub use crate::common::session_options::STATUS_RIGHT_STYLE;
pub use crate::common::session_options::STATUS_STYLE;
pub use crate::common::session_options::UPDATE_ENVIRONMENT;
pub use crate::common::session_options::VISUAL_ACTIVITY;
pub use crate::common::session_options::VISUAL_BELL;
pub use crate::common::session_options::VISUAL_SILENCE;
pub use crate::common::session_options::WORD_SEPARATORS;

// window options
pub use crate::common::window_options::ClockModeStyle;
#[cfg(feature = "tmux_2_3")]
pub use crate::common::window_options::PaneBorderStatus;
pub use crate::common::window_options::WindowOptions;
pub use crate::common::window_options::WindowOptionsBuilder;
#[cfg(feature = "tmux_2_9")]
pub use crate::common::window_options::WindowSize;
pub use crate::common::window_options::AGGRESIVE_RESIZE;
pub use crate::common::window_options::AUTOMATIC_RENAME;
pub use crate::common::window_options::AUTOMATIC_RENAME_FORMAT;
pub use crate::common::window_options::CLOCK_MODE_COLOUR;
pub use crate::common::window_options::CLOCK_MODE_STYLE;
pub use crate::common::window_options::MAIN_PANE_HEIGHT;
pub use crate::common::window_options::MAIN_PANE_WIDTH;
pub use crate::common::window_options::MODE_KEYS;
pub use crate::common::window_options::MODE_STYLE;
pub use crate::common::window_options::MONITOR_ACTIVITY;
pub use crate::common::window_options::MONITOR_BELL;
pub use crate::common::window_options::MONITOR_SILENCE;
pub use crate::common::window_options::OTHER_PANE_HEIGHT;
pub use crate::common::window_options::OTHER_PANE_WIDTH;
pub use crate::common::window_options::PANE_ACTIVE_BORDER_STYLE;
pub use crate::common::window_options::PANE_BASE_INDEX;
pub use crate::common::window_options::PANE_BORDER_FORMAT;
pub use crate::common::window_options::PANE_BORDER_STATUS;
pub use crate::common::window_options::PANE_BORDER_STYLE;
pub use crate::common::window_options::SYNCHRONIZE_PANES;
pub use crate::common::window_options::WINDOW_OPTIONS_ALL;
pub use crate::common::window_options::WINDOW_OPTIONS_NONE;
pub use crate::common::window_options::WINDOW_SIZE;
pub use crate::common::window_options::WINDOW_STATUS_ACTIVITY_STYLE;
pub use crate::common::window_options::WINDOW_STATUS_BELL_STYLE;
pub use crate::common::window_options::WINDOW_STATUS_CURRENT_FORMAT;
pub use crate::common::window_options::WINDOW_STATUS_CURRENT_STYLE;
pub use crate::common::window_options::WINDOW_STATUS_FORMAT;
pub use crate::common::window_options::WINDOW_STATUS_LAST_STYLE;
pub use crate::common::window_options::WINDOW_STATUS_SEPARATOR;
pub use crate::common::window_options::WINDOW_STATUS_STYLE;
pub use crate::common::window_options::WRAP_SEARCH;
pub use crate::common::window_options::XTERM_KEYS;

// pane options
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::PaneOptions;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::PaneOptionsBuilder;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::ALLOW_RENAME;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::ALTERNATE_SCREEN;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::PANE_OPTIONS;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::PANE_OPTIONS_ALL;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::PANE_OPTIONS_NONE;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::REMAIN_ON_EXIT;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::WINDOW_ACTIVE_STYLE;
#[cfg(feature = "tmux_3_1")]
pub use crate::common::pane_options::WINDOW_STYLE;

// buffers
#[cfg(feature = "tmux_1_3")]
pub use self::request::buffers::choose_buffer::ChooseBuffer;
#[cfg(feature = "tmux_1_3")]
pub use self::request::buffers::choose_buffer::ChooseBufferBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::buffers::paste_buffer::PasteBuffer;
#[cfg(feature = "tmux_1_0")]
pub use self::request::buffers::paste_buffer::PasteBufferBuilder;
// clients and sessions
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::attach_session::AttachSession;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::attach_session::AttachSessionBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::detach_client::DetachClient;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::detach_client::DetachClientBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::new_session::NewSession;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::new_session::NewSessionBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::refresh_client::RefreshClient;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::refresh_client::RefreshClientBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::switch_client::SwitchClient;
#[cfg(feature = "tmux_1_0")]
pub use self::request::clients_and_sessions::switch_client::SwitchClientBuilder;
// global and session environment
#[cfg(feature = "tmux_1_0")]
pub use self::request::global_and_session_environment::set_environment::SetEnvironment;
#[cfg(feature = "tmux_1_0")]
pub use self::request::global_and_session_environment::set_environment::SetEnvironmentBuilder;
// hooks
#[cfg(feature = "tmux_2_2")]
pub use self::request::hooks::set_hook::SetHook;
#[cfg(feature = "tmux_2_2")]
pub use self::request::hooks::set_hook::SetHookBuilder;
// key bindings
#[cfg(feature = "tmux_1_0")]
pub use self::request::key_bindings::send_keys::SendKeys;
#[cfg(feature = "tmux_1_0")]
pub use self::request::key_bindings::send_keys::SendKeysBuilder;
#[cfg(feature = "tmux_1_0")]
pub use crate::request::key_bindings::bind_key::BindKey;
#[cfg(feature = "tmux_1_0")]
pub use crate::request::key_bindings::bind_key::BindKeyBuilder;
// miscellaneous
#[cfg(feature = "tmux_1_0")]
pub use crate::request::miscellaneous::if_shell::IfShell;
#[cfg(feature = "tmux_1_0")]
pub use crate::request::miscellaneous::if_shell::IfShellBuilder;
// options
#[cfg(feature = "tmux_1_0")]
pub use self::request::options::set_option::SetOption;
#[cfg(feature = "tmux_1_0")]
pub use self::request::options::set_option::SetOptionBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::options::set_window_option::SetWindowOption;
#[cfg(feature = "tmux_1_0")]
pub use self::request::options::set_window_option::SetWindowOptionBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::options::show_options::ShowOptions;
#[cfg(feature = "tmux_1_0")]
pub use self::request::options::show_options::ShowOptionsBuilder;
// status line
#[cfg(feature = "tmux_1_0")]
pub use self::request::status_line::command_prompt::CommandPrompt;
#[cfg(feature = "tmux_1_0")]
pub use self::request::status_line::command_prompt::CommandPromptBuilder;
// not structure, pass as arguments
#[cfg(feature = "tmux_3_0")]
pub use self::request::status_line::display_menu::DisplayMenu;
#[cfg(feature = "tmux_3_0")]
pub use self::request::status_line::display_menu::DisplayMenuBuilder;
// not structure, pass as arguments
#[cfg(feature = "tmux_1_0")]
pub use self::request::status_line::display_message::DisplayMessage;
#[cfg(feature = "tmux_1_0")]
pub use self::request::status_line::display_message::DisplayMessageBuilder;
// windows and panes
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::break_pane::BreakPane;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::break_pane::BreakPaneBuilder;
#[cfg(feature = "tmux_1_2")]
pub use self::request::windows_and_panes::capture_pane::CapturePane;
#[cfg(feature = "tmux_1_2")]
pub use self::request::windows_and_panes::capture_pane::CapturePaneBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::choose_client::ChooseClient;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::choose_client::ChooseClientBuilder;
#[cfg(feature = "tmux_1_7")]
pub use self::request::windows_and_panes::choose_tree::ChooseTree;
#[cfg(feature = "tmux_1_7")]
pub use self::request::windows_and_panes::choose_tree::ChooseTreeBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::find_window::FindWindow;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::find_window::FindWindowBuilder;
#[cfg(feature = "tmux_1_2")]
pub use self::request::windows_and_panes::join_pane::JoinPane;
#[cfg(feature = "tmux_1_2")]
pub use self::request::windows_and_panes::join_pane::JoinPaneBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::link_window::LinkWindow;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::link_window::LinkWindowBuilder;
#[cfg(feature = "tmux_1_7")]
pub use self::request::windows_and_panes::move_pane::MovePane;
#[cfg(feature = "tmux_1_7")]
pub use self::request::windows_and_panes::move_pane::MovePaneBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::move_window::MoveWindow;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::move_window::MoveWindowBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::new_window::NewWindow;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::new_window::NewWindowBuilder;
#[cfg(feature = "tmux_1_1")]
pub use self::request::windows_and_panes::pipe_pane::PipePane;
#[cfg(feature = "tmux_1_1")]
pub use self::request::windows_and_panes::pipe_pane::PipePaneBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::resize_pane::ResizePane;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::resize_pane::ResizePaneBuilder;
#[cfg(feature = "tmux_2_9")]
pub use self::request::windows_and_panes::resize_window::ResizeWindow;
#[cfg(feature = "tmux_2_9")]
pub use self::request::windows_and_panes::resize_window::ResizeWindowBuilder;
#[cfg(feature = "tmux_1_5")]
pub use self::request::windows_and_panes::respawn_pane::RespawnPane;
#[cfg(feature = "tmux_1_5")]
pub use self::request::windows_and_panes::respawn_pane::RespawnPaneBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::respawn_window::RespawnWindow;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::respawn_window::RespawnWindowBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::select_layout::SelectLayot;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::select_layout::SelectLayotBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::select_pane::SelectPane;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::select_pane::SelectPaneBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::select_window::SelectWindow;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::select_window::SelectWindowBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::split_window::SplitWindow;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::split_window::SplitWindowBuilder;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::swap_pane::SwapPane;
#[cfg(feature = "tmux_1_0")]
pub use self::request::windows_and_panes::swap_pane::SwapPaneBuilder;

// enums
pub use crate::request::PaneSize;

pub use crate::request::target_pane::{TargetPane, TargetPaneExt, TargetPaneToken};
pub use crate::request::target_session::TargetSession;
pub use crate::request::target_window::{TargetWindow, TargetWindowExt, TargetWindowToken};

// structs
pub use self::tmux_option::TmuxOption;

// enums
pub use self::response::layout::layout_cell::LayoutType;
// structs
pub use self::response::layout::layout::Layout;
pub use self::response::layout::layout_cell::LayoutCell;
pub use self::response::layout::layout_checksum::LayoutChecksum;
pub use self::response::pane::pane::Pane;
pub use self::response::pane::pane_tabs::PaneTabs;
pub use self::response::pane::panes::Panes;
pub use self::response::session::session::Session;
pub use self::response::session::session_stack::SessionStack;
pub use self::response::session::sessions::Sessions;
pub use self::response::window::window::Window;
pub use self::response::window::window_flag::WindowFlag;
pub use self::response::window::windows::Windows;
pub use self::version::Version;

// structs
pub use self::error::Error;

mod tmux_interface_tests;
//mod options_tests;
mod tmux_option_tests;
mod version_tests;

// consts
pub use crate::response::session::session::SESSION_ACTIVITY;
pub use crate::response::session::session::SESSION_ALERTS;
pub use crate::response::session::session::SESSION_ATTACHED;
pub use crate::response::session::session::SESSION_CREATED;
pub use crate::response::session::session::SESSION_FORMAT;
pub use crate::response::session::session::SESSION_GROUP;
pub use crate::response::session::session::SESSION_GROUPED;
pub use crate::response::session::session::SESSION_GROUP_LIST;
pub use crate::response::session::session::SESSION_GROUP_SIZE;
pub use crate::response::session::session::SESSION_ID;
pub use crate::response::session::session::SESSION_LAST_ATTACHED;
pub use crate::response::session::session::SESSION_MANY_ATTACHED;
pub use crate::response::session::session::SESSION_NAME;
pub use crate::response::session::session::SESSION_STACK;
pub use crate::response::session::session::SESSION_WINDOWS;

pub use crate::response::session::session::SESSION_NONE;
//pub use crate::response::session::session::SESSION_DEFAULT;
pub use crate::response::session::session::SESSION_ALL;

pub use crate::response::window::window::WINDOW_ACTIVE;
pub use crate::response::window::window::WINDOW_ACTIVITY;
pub use crate::response::window::window::WINDOW_ACTIVITY_FLAG;
pub use crate::response::window::window::WINDOW_BELL_FLAG;
pub use crate::response::window::window::WINDOW_BIGGER;
pub use crate::response::window::window::WINDOW_END_FLAG;
pub use crate::response::window::window::WINDOW_FLAGS;
pub use crate::response::window::window::WINDOW_FORMAT;
pub use crate::response::window::window::WINDOW_HEIGHT;
pub use crate::response::window::window::WINDOW_ID;
pub use crate::response::window::window::WINDOW_INDEX;
pub use crate::response::window::window::WINDOW_LAST_FLAG;
pub use crate::response::window::window::WINDOW_LAYOUT;
pub use crate::response::window::window::WINDOW_LINKED;
pub use crate::response::window::window::WINDOW_NAME;
pub use crate::response::window::window::WINDOW_OFFSET_X;
pub use crate::response::window::window::WINDOW_OFFSET_Y;
pub use crate::response::window::window::WINDOW_PANES;
pub use crate::response::window::window::WINDOW_SILENCE_FLAG;
pub use crate::response::window::window::WINDOW_STACK_INDEX;
pub use crate::response::window::window::WINDOW_START_FLAG;
pub use crate::response::window::window::WINDOW_VISIBLE_LAYOUT;
pub use crate::response::window::window::WINDOW_WIDTH;
pub use crate::response::window::window::WINDOW_ZOOMED_FLAG;

pub use crate::response::window::window::WINDOW_NONE;
//pub use crate::response::window::window::WINDOW_DEFAULT;
pub use crate::response::window::window::WINDOW_ALL;

pub use crate::response::pane::pane::PANE_ACTIVE;
pub use crate::response::pane::pane::PANE_AT_BOTTOM;
pub use crate::response::pane::pane::PANE_AT_LEFT;
pub use crate::response::pane::pane::PANE_AT_RIGHT;
pub use crate::response::pane::pane::PANE_AT_TOP;
pub use crate::response::pane::pane::PANE_BOTTOM;
pub use crate::response::pane::pane::PANE_CURRENT_COMMAND;
pub use crate::response::pane::pane::PANE_CURRENT_PATH;
pub use crate::response::pane::pane::PANE_DEAD;
pub use crate::response::pane::pane::PANE_DEAD_STATUS;
pub use crate::response::pane::pane::PANE_FORMAT;
pub use crate::response::pane::pane::PANE_HEIGHT;
pub use crate::response::pane::pane::PANE_ID;
pub use crate::response::pane::pane::PANE_INDEX;
pub use crate::response::pane::pane::PANE_INPUT_OFF;
pub use crate::response::pane::pane::PANE_IN_MODE;
pub use crate::response::pane::pane::PANE_LEFT;
pub use crate::response::pane::pane::PANE_MARKED;
pub use crate::response::pane::pane::PANE_MARKED_SET;
pub use crate::response::pane::pane::PANE_MODE;
pub use crate::response::pane::pane::PANE_PID;
pub use crate::response::pane::pane::PANE_PIPE;
pub use crate::response::pane::pane::PANE_RIGHT;
pub use crate::response::pane::pane::PANE_SEARCH_STRING;
pub use crate::response::pane::pane::PANE_START_COMMMAND;
pub use crate::response::pane::pane::PANE_SYNCHRONIZED;
pub use crate::response::pane::pane::PANE_TABS;
pub use crate::response::pane::pane::PANE_TITLE;
pub use crate::response::pane::pane::PANE_TOP;
pub use crate::response::pane::pane::PANE_TTY;
pub use crate::response::pane::pane::PANE_WIDTH;

pub use crate::response::pane::pane::PANE_NONE;
//pub use crate::response::pane::pane::PANE_DEFAULT;
pub use crate::response::pane::pane::PANE_ALL;
