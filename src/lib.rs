//! **tmux_interface** as a Rust library provides functionality to communicate with TMUX via CLI.
//!
//! # Description
//!
//! Main purpose of the tmux_interface is to implement simple sending and recieving data mechanisms
//! for some Rust application, using intercommunication with TMUX only via standard streams (stdin,
//! stdout, stderr).
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
//! # Library Functions
//!
//! 1. Function names and their grouping are inherited from tmux manual
//! 2. Function arguments and their optionality inherited from tmux manual
//! 3. Functions can have max. 4 arguments, otherwise a structure will be used
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::{TmuxInterface, AttachSession, NewSession};
//!
//!
//! fn main() {
//!     let mut tmux = TmuxInterface::new();
//!
//!     let new_session = NewSession {
//!         detached: Some(true),
//!         session_name: Some("test_session_name1"),
//!         ..Default::default()
//!     };
//!     tmux.new_session(Some(&new_session)).unwrap();
//!        let attach_session = AttachSession {
//!        target_session: Some("test_session_name1"),
//!        ..Default::default()
//!     };
//!     tmux.attach_session(Some(&attach_session)).unwrap();
//!     tmux.kill_session(None, None, Some("test_session_name1")).unwrap();
//!
//!     // or alternatively
//!     let mut new_session = NewSession::new();
//!     new_session.detached = Some(true);
//!     new_session.session_name = Some("test_session_name2");
//!     tmux.new_session(Some(&new_session)).unwrap();
//!     let mut attach_session = AttachSession::new();
//!     attach_session.target_session = Some("test_session_name2");
//!     tmux.kill_session(None, None, Some("test_session_name2")).unwrap();
//! }
//! ```
//!
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::{Sessions, Session, Windows, Window, Pane, Panes};
//! use crate::tmux_interface::session::SESSION_ALL;
//! use crate::tmux_interface::window::WINDOW_ALL;
//! use crate::tmux_interface::pane::PANE_ALL;
//!
//! fn main() {
//!     let sessions = Sessions::get(SESSION_ALL).unwrap();
//!     let windows = Windows::get("0", WINDOW_ALL).unwrap();
//!     let panes = Panes::get("0:1", PANE_ALL).unwrap();
//! }
//! ```
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::{TmuxInterface, NewSession};
//!
//! fn main() {
//!     let mut tmux = TmuxInterface::new();
//!     tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
//!         // changing of binary name, its arguments, subcommand and its parameters are allowed
//!         // inside callback function
//!         *bin = "tmux".to_string();
//!         // display newly set variables
//!         println!("pre hook: {:?} {:?} {:?}", bin, options, subcmd);
//!         //Err(Error::new("pre hook error"))
//!         Ok(())
//!     }));
//!
//!     let new_session = NewSession {
//!         detached: Some(true),
//!         session_name: Some("test_session_name1"),
//!         ..Default::default()
//!     };
//!     tmux.new_session(Some(&new_session)).unwrap();
//!     tmux.pre_hook = None;
//!     tmux.kill_session(None, None, Some("test_session_name1")).unwrap();
//! }
//! ```

pub mod buffers;
pub mod clients_and_sessions;
pub mod error;
pub mod global_and_session_environment;
pub mod hooks;
pub mod key_bindings;
pub mod layout;
pub mod layout_cell;
pub mod layout_checksum;
pub mod miscellaneous;
pub mod options;
pub mod pane;
pub mod pane_tabs;
pub mod panes;
pub mod session;
pub mod session_stack;
pub mod sessions;
pub mod status_line;
pub mod tmux_interface;
pub mod tmux_option;
pub mod version;
pub mod window;
pub mod window_flag;
pub mod windows;
pub mod windows_and_panes;

// structs
pub use self::clients_and_sessions::AttachSession;
pub use self::clients_and_sessions::DetachClient;
pub use self::clients_and_sessions::NewSession;
pub use self::clients_and_sessions::RefreshClient;
pub use self::clients_and_sessions::SwitchClient;
pub use self::tmux_interface::TmuxInterface;

// enums
pub use self::windows_and_panes::PaneSize;
// structs
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

// structs
pub use self::key_bindings::BindKey;
pub use self::key_bindings::SendKeys;

// structs
pub use self::options::ShowOptions;
pub use self::tmux_option::TmuxOption;

// enums
pub use self::layout_cell::LayoutType;
// structs
pub use self::layout::Layout;
pub use self::layout_cell::LayoutCell;
pub use self::layout_checksum::LayoutChecksum;
pub use self::pane::Pane;
pub use self::pane_tabs::PaneTabs;
pub use self::panes::Panes;
pub use self::session::Session;
pub use self::session_stack::SessionStack;
pub use self::sessions::Sessions;
pub use self::version::Version;
pub use self::window::Window;
pub use self::window_flag::WindowFlag;
pub use self::windows::Windows;

// structs
pub use self::error::Error;

//mod pane_tabs_tests;
mod pane_tests;
mod panes_tests;
//mod session_stack_tests;
mod session_tests;
mod sessions_tests;
#[cfg(test)]
mod tmux_interface_tests;
//mod window_flag_tests;
mod window_tests;
mod windows_tests;
//mod options_tests;
mod layout_cell_tests;
mod layout_checksum_tests;
mod layout_tests;
mod tmux_option_tests;
mod version_tests;
