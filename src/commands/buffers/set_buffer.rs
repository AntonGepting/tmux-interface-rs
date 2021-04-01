use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Set the contents of the specified buffer to data.
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// tmux set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
/// (alias: setb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux set-buffer [-b buffer-index] data
/// (alias: setb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux set-buffer [-b buffer-index] [-t target-session] data
/// (alias: setb)
/// ```
#[derive(Debug, Clone)]
pub struct SetBuffer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SetBuffer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SET_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> SetBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_2_0")]
    pub fn append(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(&mut self, buffer_name: S) -> &mut Self {
        self.0.push_option(B_LOWERCASE_KEY, buffer_name);
        self
    }

    /// `[-n new-buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn new_buffer_name<S: Into<Cow<'a, str>>>(&mut self, new_buffer_name: S) -> &mut Self {
        self.0.push_option(N_LOWERCASE_KEY, new_buffer_name);
        self
    }

    /// `data`
    #[cfg(feature = "tmux_0_8")]
    pub fn data<S: Into<Cow<'a, str>>>(&mut self, data: S) -> &mut Self {
        self.0.push_param(data);
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
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

impl<'a> From<TmuxCommand<'a>> for SetBuffer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SET_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SetBuffer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SET_BUFFER)),
            ..Default::default()
        })
    }
}
