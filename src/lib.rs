pub mod tmux_interface;
pub mod session;
pub mod window;
pub mod pane;
pub mod tmux_interface_error;
pub mod windows_and_panes;
pub mod clients_and_sessions;
pub mod key_bindings;
pub mod status_line;


pub use self::tmux_interface::TmuxInterface;
pub use self::clients_and_sessions::NewSession;
pub use self::clients_and_sessions::AttachSession;
pub use self::windows_and_panes::NewWindow;
pub use self::windows_and_panes::SplitWindow;
pub use self::windows_and_panes::SelectPane;
pub use self::windows_and_panes::SelectWindow;
pub use self::key_bindings::SendKeys;
pub use self::session::Session;
pub use self::session::Sessions;
pub use self::session::LIST_SESSIONS_FORMAT;
pub use self::window::Window;
pub use self::window::Windows;
pub use self::window::LIST_WINDOWS_FORMAT;
pub use self::pane::Pane;
pub use self::pane::Panes;
pub use self::pane::LIST_PANES_FORMAT;
pub use self::tmux_interface_error::TmuxInterfaceError;


#[cfg(test)]
mod tmux_interface_tests;
mod clients_and_sessions_tests;
mod windows_and_panes_tests;
