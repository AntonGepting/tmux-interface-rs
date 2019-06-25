//! **tmux_interface** as a Rust library provides functionality to communicate with TMUX via CLI.
//!
//! Main purpose of the tmux_interface is to implement simple sending and recieving data mechanisms
//! for some Rust application, using intercommunication with TMUX only via standard streams (stdin,
//! stdout, stderr).
//!
//! This goal can be reached by splitting it into two separate tasks:
//!
//! 1. Providing wrapper functions for tmux subcommands (sending data). Wrapper functions are
//! structured like in tmux manual in few next categories:
//!     - Clients and Sessions
//!     - Windows and Panes
//!     - Key Bindings
//!     - Options
//!     - Hooks
//!     - Global and Session Environment
//!     - Status Line
//!     - Buffers
//!     - Miscellaneous
//!
//! 2. Parsing functions for tmux output as rust structures (recieving data). Parsing function are
//! structured by objects they operate with:
//!     - Sessions
//!     - Session
//!     - Windows
//!     - Window
//!     - Panes
//!     - Pane
//!     - ...
//!     - Option
//!
//! Example:
//!
//! ```
//! use crate::tmux_interface::TmuxInterface;
//! use crate::tmux_interface::NewSession;
//!
//!
//! fn main() {
//!     let tmux = TmuxInterface::new();
//!
//!     let new_session = NewSession {
//!         detached: Some(true),
//!         session_name: Some("test_session_name1"),
//!         ..Default::default()
//!     };
//!     tmux.new_session(&new_session).unwrap();
//!
//!     // or alternatively
//!     let new_session = NewSession::new();
//!     new_session.detached = Some(true);
//!     new_session.session_name = Some("test_session_name2");
//!     tmux.new_session(&new_session).unwrap();
//! }
//! ```
//!


pub mod tmux_interface;
pub mod session;
pub mod sessions;
pub mod window;
pub mod windows;
pub mod pane;
pub mod panes;
pub mod tmux_interface_error;
pub mod windows_and_panes;
pub mod clients_and_sessions;
pub mod key_bindings;
pub mod status_line;
pub mod options;
pub mod hooks;
pub mod buffers;
pub mod global_and_session_environment;
pub mod miscellaneous;
pub mod tmux_option;


pub use self::tmux_interface::TmuxInterface;
pub use self::clients_and_sessions::AttachSession;
pub use self::clients_and_sessions::DetachClient;
pub use self::clients_and_sessions::NewSession;
pub use self::clients_and_sessions::RefreshClient;
pub use self::clients_and_sessions::SwitchClient;

pub use self::windows_and_panes::BreakPane;
pub use self::windows_and_panes::CapturePane;
pub use self::windows_and_panes::ChooseClient;
pub use self::windows_and_panes::ChooseTree;
pub use self::windows_and_panes::FindWindow;
pub use self::windows_and_panes::JoinPane;
pub use self::windows_and_panes::LinkWindow;
pub use self::windows_and_panes::MovePane;
pub use self::windows_and_panes::MoveWindow;
pub use self::windows_and_panes::NewWindow;
pub use self::windows_and_panes::PipePane;
pub use self::windows_and_panes::ResizePane;
pub use self::windows_and_panes::ResizeWindow;
pub use self::windows_and_panes::RespawnPane;
pub use self::windows_and_panes::RespawnWindow;
pub use self::windows_and_panes::SelectLayot;
pub use self::windows_and_panes::SelectPane;
pub use self::windows_and_panes::SelectWindow;
pub use self::windows_and_panes::SplitWindow;
pub use self::windows_and_panes::SwapPane;

pub use self::key_bindings::BindKey;
pub use self::key_bindings::SendKeys;

pub use self::options::ShowOptions;
pub use self::tmux_option::TmuxOption;

pub use self::session::Session;
pub use self::sessions::Sessions;
pub use self::window::Window;
pub use self::windows::Windows;
pub use self::pane::Pane;
pub use self::panes::Panes;

pub use self::tmux_interface_error::TmuxInterfaceError;


#[cfg(test)]
mod tmux_interface_tests;
mod session_tests;
mod sessions_tests;
mod window_tests;
mod windows_tests;
mod pane_tests;
mod panes_tests;
//mod options_tests;
mod tmux_option_tests;
