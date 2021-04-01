use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// List the global buffers.
///
/// # Manual
///
/// tmux ^1.7:
/// ```text
/// tmux list-buffers [-F format]
/// (alias: lsb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux list-buffers
/// (alias: lsb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-buffers [-t target-session]
/// (alias: lsb)
/// ```
#[derive(Debug, Clone)]
pub struct ListBuffers<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ListBuffers<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_BUFFERS)),
            ..Default::default()
        })
    }
}

impl<'a> ListBuffers<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ListBuffers<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LIST_BUFFERS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ListBuffers<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LIST_BUFFERS)),
            ..Default::default()
        })
    }
}
