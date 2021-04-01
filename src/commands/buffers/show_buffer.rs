use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Display the contents of the specified buffer.
///
/// # Manual
///
/// tmux ^1.5:
/// ```text
/// tmux show-buffer [-b buffer-name]
/// (alias: showb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux show-buffer [-b buffer-index] [-t target-session]
/// (alias: showb)
/// ```
#[derive(Debug, Clone)]
pub struct ShowBuffer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ShowBuffer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SHOW_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> ShowBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(&mut self, buffer_name: S) -> &mut Self {
        self.0.push_option(B_LOWERCASE_KEY, buffer_name);
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn buffer_index<S: Into<Cow<'a, str>>>(&mut self, buffer_index: S) -> &mut Self {
        self.0.push_option(B_LOWERCASE_KEY, buffer_index);
        self
    }

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

impl<'a> From<TmuxCommand<'a>> for ShowBuffer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SHOW_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ShowBuffer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SHOW_BUFFER)),
            ..Default::default()
        })
    }
}
