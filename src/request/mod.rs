pub mod buffers;
pub mod clients_and_sessions;
pub mod global_and_session_environment;
pub mod hooks;
pub mod key_bindings;
pub mod miscellaneous;
pub mod options;
pub mod status_line;
pub mod windows_and_panes;

pub mod target_pane;
pub mod target_session;
pub mod target_window;

pub mod target_pane_tests;
pub mod target_session_tests;
pub mod target_window_tests;

// common for mod
#[derive(Debug)]
pub enum PaneSize {
    Size(usize),
    Percentage(usize),
}
