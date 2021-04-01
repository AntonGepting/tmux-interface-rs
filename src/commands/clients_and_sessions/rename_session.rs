use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Rename the session to `new-name`
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux rename-session [-t target-session] new-name
/// (alias: rename)
/// ```
#[derive(Debug, Clone)]
pub struct RenameSession<'a>(pub TmuxCommand<'a>);

impl<'a> Default for RenameSession<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(RENAME_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> RenameSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    /// `new-name`
    #[cfg(feature = "tmux_0_8")]
    pub fn new_name<S: Into<Cow<'a, str>>>(&mut self, new_name: S) -> &mut Self {
        self.0.push_param(new_name);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for RenameSession<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(RENAME_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for RenameSession<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(RENAME_SESSION)),
            ..Default::default()
        })
    }
}
