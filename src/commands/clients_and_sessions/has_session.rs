use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

// XXX: better result return?
/// Report if the specified session exist
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux has-session [-t target-session]
/// (alias: has)
/// ```
#[derive(Debug)]
pub struct HasSession<'a>(pub TmuxCommand<'a>);

impl<'a> Default for HasSession<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(HAS_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> HasSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    /// run command
    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for HasSession<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(HAS_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for HasSession<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(HAS_SESSION)),
            ..Default::default()
        })
    }
}
