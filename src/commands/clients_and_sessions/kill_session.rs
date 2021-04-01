use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Destroy the given session
///
/// # Manual
///
/// tmux ^2.2:
/// ```text
/// tmux kill-session [-aC] [-t target-session]
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux kill-session [-a] [-t target-session]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux kill-session [-t target-session]
/// ```
#[derive(Debug, Clone)]
pub struct KillSession<'a>(pub TmuxCommand<'a>);

impl<'a> Default for KillSession<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(KILL_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> KillSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_2_2")]
    pub fn all(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-C]`
    #[cfg(feature = "tmux_1_7")]
    pub fn clear_alerts(&mut self) -> &mut Self {
        self.0.push_flag(C_UPPERCASE_KEY);
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for KillSession<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(KILL_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for KillSession<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(KILL_SESSION)),
            ..Default::default()
        })
    }
}
