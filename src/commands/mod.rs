pub mod constants;
pub mod tmux;
pub mod tmux_command;
pub mod tmux_commands;
pub mod tmux_output;

pub use self::tmux::Tmux;
use crate::TmuxCommand;

pub mod tmux_command_tests;
pub mod tmux_commands_tests;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PaneSize {
    Size(usize),
    Percentage(usize),
}

/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html))
impl<'a> TmuxCommand<'a> {
    pub fn tmux() -> Tmux<'a> {
        Tmux::new()
    }
}
