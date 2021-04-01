use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Delete the buffer named buffer-name, or the most recently added automatically named buffer
/// if not specified.
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// tmux delete-buffer [-b buffer-name]
/// (alias: deleteb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux delete-buffer [-b buffer-index]
/// (alias: deleteb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux delete-buffer [-b buffer-index] [-t target-session]
/// (alias: deleteb)
/// ```
#[derive(Debug, Clone)]
pub struct DeleteBuffer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for DeleteBuffer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(DELETE_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> DeleteBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_0_8")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(&mut self, buffer_name: S) -> &mut Self {
        self.0.push_option(B_LOWERCASE_KEY, buffer_name);
        self
    }

    // FIXME: buffer-index

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for DeleteBuffer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(DELETE_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for DeleteBuffer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(DELETE_BUFFER)),
            ..Default::default()
        })
    }
}
