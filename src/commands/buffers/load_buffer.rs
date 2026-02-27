// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type LoadB<'a> = LoadBuffer<'a>;

/// Load the contents of the specified paste buffer from path.
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// load-buffer [-w] [-b buffer-name] [-t target-client] path
/// (alias: loadb)
/// ```
///
/// tmux >=2.0:
/// ```text
/// load-buffer [-b buffer-name] path
/// (alias: loadb)
/// ```
///
/// tmux >=1.5:
/// ```text
/// load-buffer [-b buffer-index] path
/// (alias: loadb)
/// ```
///
/// tmux >=0.8:
/// ```text
/// load-buffer [-b buffer-index] [-t target-session] path
/// (alias: loadb)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LoadBuffer<'a> {
    /// `[-w]`
    #[cfg(feature = "tmux_3_2")]
    pub send_to_clipboard: bool,

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub buffer_index: Option<Cow<'a, str>>,

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub buffer_name: Option<Cow<'a, str>>,

    /// `[-t target-client]`
    #[cfg(feature = "tmux_3_2")]
    pub target_client: Option<Cow<'a, str>>,

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
    pub fn send_to_clipboard(mut self) -> Self {
        self.send_to_clipboard = true;
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub fn buffer_index<S: Into<Cow<'a, str>>>(mut self, buffer_index: S) -> Self {
        self.buffer_index = Some(buffer_index.into());
        self
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(mut self, buffer_name: S) -> Self {
        self.buffer_name = Some(buffer_name.into());
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_3_2")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[path]`
    #[cfg(feature = "tmux_0_8")]
    pub fn path<S: Into<Cow<'a, str>>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LOAD_BUFFER);

        // `[-w]`
        #[cfg(feature = "tmux_3_2")]
        if self.send_to_clipboard {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-b buffer-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
        if let Some(buffer_index) = self.buffer_index {
            cmd.push_option(B_LOWERCASE_KEY, buffer_index);
        }

        // `[-b buffer-name]`
        #[cfg(feature = "tmux_2_0")]
        if let Some(buffer_name) = self.buffer_name {
            cmd.push_option(B_LOWERCASE_KEY, buffer_name);
        }

        // `[-t target-client]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client);
        }

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `[path]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(path) = self.path {
            cmd.push_param(path);
        }

        cmd
    }
}
