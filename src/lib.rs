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
//! use crate::tmux_interface::response::session::session::SESSION_ALL;
//! use crate::tmux_interface::response::window::window::WINDOW_ALL;
//! use crate::tmux_interface::response::pane::pane::PANE_ALL;
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
//!         Ok(None)
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

pub mod error;

pub mod request;
pub mod response;

pub mod tmux_interface;
pub mod tmux_option;
pub mod version;

pub use self::tmux_interface::TmuxInterface;

// buffers
pub use self::request::buffers::choose_buffer::ChooseBuffer;
pub use self::request::buffers::paste_buffer::PasteBuffer;
// clients and sessions
pub use self::request::clients_and_sessions::attach_session::AttachSession;
pub use self::request::clients_and_sessions::detach_client::DetachClient;
pub use self::request::clients_and_sessions::new_session::NewSession;
#[cfg(not(feature = "tmux_2_6"))]
pub use self::request::clients_and_sessions::refresh_client::RefreshClient;
pub use self::request::clients_and_sessions::switch_client::SwitchClient;
// global and session environment
pub use self::request::global_and_session_environment::set_environment::SetEnvironment;
// hooks
pub use self::request::hooks::set_hook::SetHook;
// key bindings
pub use self::request::key_bindings::send_keys::SendKeys;
pub use crate::request::key_bindings::bind_key::BindKey;
// miscellaneous
pub use crate::request::miscellaneous::if_shell::IfShell;
// options
pub use self::request::options::show_options::ShowOptions;
// status line
pub use self::request::status_line::command_prompt::CommandPrompt;
#[cfg(not(feature = "tmux_2_6"))]
pub use self::request::status_line::display_menu::DisplayMenu;
#[cfg(not(feature = "tmux_2_6"))]
pub use self::request::status_line::display_message::DisplayMessage;
// windows and panes
pub use self::request::windows_and_panes::break_pane::BreakPane;
pub use self::request::windows_and_panes::capture_pane::CapturePane;
pub use self::request::windows_and_panes::choose_client::ChooseClient;
pub use self::request::windows_and_panes::choose_tree::ChooseTree;
pub use self::request::windows_and_panes::find_window::FindWindow;
pub use self::request::windows_and_panes::join_pane::JoinPane;
pub use self::request::windows_and_panes::link_window::LinkWindow;
pub use self::request::windows_and_panes::move_pane::MovePane;
pub use self::request::windows_and_panes::move_window::MoveWindow;
pub use self::request::windows_and_panes::new_window::NewWindow;
#[cfg(not(feature = "tmux_2_6"))]
pub use self::request::windows_and_panes::pipe_pane::PipePane;
pub use self::request::windows_and_panes::resize_pane::ResizePane;
pub use self::request::windows_and_panes::resize_window::ResizeWindow;
#[cfg(not(feature = "tmux_2_6"))]
pub use self::request::windows_and_panes::respawn_pane::RespawnPane;
#[cfg(not(feature = "tmux_2_6"))]
pub use self::request::windows_and_panes::respawn_window::RespawnWindow;
pub use self::request::windows_and_panes::select_layout::SelectLayot;
pub use self::request::windows_and_panes::select_pane::SelectPane;
pub use self::request::windows_and_panes::select_window::SelectWindow;
pub use self::request::windows_and_panes::split_window::SplitWindow;
pub use self::request::windows_and_panes::swap_pane::SwapPane;

// enums
//pub use crate::windows_and_panes::PaneSize;

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

//mod pane_tabs_tests;
//mod pane_tests;
//mod panes_tests;
//mod session_stack_tests;
//mod session_tests;
//mod sessions_tests;
mod tmux_interface_tests;
//mod window_flag_tests;
//mod window_tests;
//mod windows_tests;
//mod options_tests;
//mod layout_cell_tests;
//mod layout_checksum_tests;
//mod layout_tests;
mod tmux_option_tests;
mod version_tests;

//mod windows_and_panes_tests;
