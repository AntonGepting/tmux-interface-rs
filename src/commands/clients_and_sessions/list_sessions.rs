use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

// XXX: better result return?
/// List all sessions managed by the server
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// tmux list-sessions [-F format]
/// (alias: ls)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-sessions
/// (alias: ls)
/// ```
/// [-t target-session]
#[derive(Debug, Clone)]
pub struct ListSessions<'a>(TmuxCommand<'a>);

impl<'a> Default for ListSessions<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_SESSIONS)),
            ..Default::default()
        })
    }
}

impl<'a> ListSessions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<String>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_KEY, format);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ListSessions<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LIST_SESSIONS)),
            ..Default::default()
        })
    }
}
