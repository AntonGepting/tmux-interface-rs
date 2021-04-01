use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

// XXX: better return type
/// List windows on the server
///
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// tmux list-windows [-a] [-F format] [-t target-session]
/// (alias: lsw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux list-windows [-a] [-t target-session]
/// (alias: lsw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-windows [-t target-session]
/// (alias: lsw)
/// ```
#[derive(Debug, Clone)]
pub struct ListWindows<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ListWindows<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_WINDOWS)),
            ..Default::default()
        })
    }
}

impl<'a> ListWindows<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub fn all(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
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

impl<'a> From<TmuxCommand<'a>> for ListWindows<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LIST_WINDOWS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ListWindows<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LIST_WINDOWS)),
            ..Default::default()
        })
    }
}
