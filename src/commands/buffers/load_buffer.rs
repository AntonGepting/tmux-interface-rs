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
#[derive(Debug, Default, Clone)]
pub struct LoadBuffer<'a> {
    /// `[-w]`
    #[cfg(feature = "tmux_3_2")]
    pub send_to_clipboard: Option<bool>,

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub buffer_name: Option<Cow<'a, str>>,

    /// `[-t target-client]`
    #[cfg(feature = "tmux_3_2")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub buffer_index: Option<Cow<'a, str>>,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_session: Option<Cow<'a, str>>,

    /// `[path]`
    #[cfg(feature = "tmux_0_8")]
    pub path: Option<Cow<'a, str>>,
}

impl<'a> LoadBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-w]`
    #[cfg(feature = "tmux_3_2")]
    pub fn send_to_clipboard(&mut self) -> &mut Self {
        self.path = Some(true);
        self
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(&mut self, buffer_name: S) -> &mut Self {
        self.buffer_name = Some(buffer_name.into());
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_3_2")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub fn buffer_index<S: Into<Cow<'a, str>>>(&mut self, buffer_index: S) -> &mut Self {
        self.buffer_index = Some(buffer_index.into());
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[path]`
    #[cfg(feature = "tmux_0_8")]
    pub fn path<S: Into<Cow<'a, str>>>(&mut self, path: S) -> &mut Self {
        self.path = Some(path.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LOAD_BUFFER);

        // `[-w]`
        #[cfg(feature = "tmux_3_2")]
        if self.send_to_clipboard.is_some() {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-b buffer-name]`
        #[cfg(feature = "tmux_2_0")]
        if let Some(buffer_name) = &self.buffer_name {
            cmd.push_option(B_LOWERCASE_KEY, buffer_name.as_ref());
        }

        // `[-t target-client]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_client) = &self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client.as_ref());
        }

        // `[-b buffer-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
        if let Some(buffer_index) = &self.buffer_index {
            cmd.push_option(B_LOWERCASE_KEY, buffer_index.as_ref());
        }

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_session) = &self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session.as_ref());
        }

        // `[path]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(path) = &self.path {
            cmd.push_param(path.as_ref());
        }

        cmd
    }
}
