use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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
#[derive(Debug, Clone)]
pub struct ListSessions<'a>(pub TmuxCommand<'a>);

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

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
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

impl<'a> From<&TmuxCommand<'a>> for ListSessions<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LIST_SESSIONS)),
            ..Default::default()
        })
    }
}
