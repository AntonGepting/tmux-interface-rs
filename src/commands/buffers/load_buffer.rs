use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Load the contents of the specified paste buffer from path.
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux load-buffer [-w] [-b buffer-name] [-t target-client] path
/// (alias: loadb)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux load-buffer [-b buffer-name] path
/// (alias: loadb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux load-buffer [-b buffer-index] path
/// (alias: loadb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux load-buffer [-b buffer-index] [-t target-session] path
/// (alias: loadb)
/// ```
#[derive(Debug, Clone)]
pub struct LoadBuffer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for LoadBuffer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LOAD_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> LoadBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-w]`
    #[cfg(feature = "tmux_3_2")]
    pub fn send_to_clipboard(&mut self) -> &mut Self {
        self.0.push_flag(W_LOWERCASE_KEY);
        self
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(&mut self, buffer_name: S) -> &mut Self {
        self.0.push_option(B_LOWERCASE_KEY, buffer_name);
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_3_2")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_client);
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

    /// `[path]`
    #[cfg(feature = "tmux_0_8")]
    pub fn path<S: Into<Cow<'a, str>>>(&mut self, path: S) -> &mut Self {
        self.0.push_param(path);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for LoadBuffer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LOAD_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for LoadBuffer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LOAD_BUFFER)),
            ..Default::default()
        })
    }
}
