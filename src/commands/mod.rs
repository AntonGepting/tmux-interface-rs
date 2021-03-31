pub mod constants;
pub mod tmux;
pub mod tmux_command;
pub mod tmux_output;

pub mod tmux_tests;

pub mod buffers;
pub mod clients_and_sessions;
pub mod global_and_session_environment;
pub mod hooks;
pub mod key_bindings;
pub mod miscellaneous;
pub mod options;
pub mod status_line;
pub mod windows_and_panes;

// common for mod
#[derive(Debug)]
pub enum PaneSize {
    Size(usize),
    Percentage(usize),
}
