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


pub use self::tmux_interface::TmuxInterface;
pub use self::clients_and_sessions::NewSession;
pub use self::clients_and_sessions::AttachSession;
pub use self::windows_and_panes::NewWindow;
pub use self::windows_and_panes::SplitWindow;
pub use self::windows_and_panes::SelectPane;
pub use self::windows_and_panes::SelectWindow;
pub use self::key_bindings::SendKeys;
pub use self::session::Session;
pub use self::sessions::Sessions;
pub use self::session::LIST_SESSIONS_FORMAT;
pub use self::window::Window;
pub use self::windows::Windows;
pub use self::window::LIST_WINDOWS_FORMAT;
pub use self::pane::Pane;
pub use self::panes::Panes;
pub use self::pane::LIST_PANES_FORMAT;
pub use self::tmux_interface_error::TmuxInterfaceError;


#[cfg(test)]
mod tmux_interface_tests;
mod session_tests;
mod sessions_tests;
mod window_tests;
mod windows_tests;
mod pane_tests;
mod panes_tests;
