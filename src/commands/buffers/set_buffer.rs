use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Set the contents of the specified buffer to data.
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// set-buffer [-aw] [-b buffer-name] [-t target-client] [-n new-buffer-name] data
/// (alias: setb)
/// ```
///
/// tmux ^2.0:
/// ```text
/// set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
/// (alias: setb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// set-buffer [-b buffer-index] data
/// (alias: setb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// set-buffer [-b buffer-index] [-t target-session] data
/// (alias: setb)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SetBuffer<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_2_0")]
    pub append: bool,

    /// `[-w]`
    #[cfg(feature = "tmux_3_2")]
    pub send_to_clipboard: bool,

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub buffer_name: Option<Cow<'a, str>>,

    /// `[-t target-client]`
    #[cfg(feature = "tmux_3_2")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-n new-buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub new_buffer_name: Option<Cow<'a, str>>,

    /// `data`
    #[cfg(feature = "tmux_0_8")]
    pub data: Option<Cow<'a, str>>,

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub buffer_index: Option<Cow<'a, str>>,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> SetBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_2_0")]
    pub fn append(mut self) -> Self {
        self.append = true;
        self
    }

    /// `[-w]`
    #[cfg(feature = "tmux_3_2")]
    pub fn send_to_clipboard(mut self) -> Self {
        self.send_to_clipboard = true;
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

    /// `[-n new-buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn new_buffer_name<S: Into<Cow<'a, str>>>(mut self, new_buffer_name: S) -> Self {
        self.new_buffer_name = Some(new_buffer_name.into());
        self
    }

    /// `data`
    #[cfg(feature = "tmux_0_8")]
    pub fn data<S: Into<Cow<'a, str>>>(mut self, data: S) -> Self {
        self.data = Some(data.into());
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    pub fn buffer_index<S: Into<Cow<'a, str>>>(mut self, buffer_index: S) -> Self {
        self.buffer_index = Some(buffer_index.into());
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SET_BUFFER);

        // `[-a]`
        #[cfg(feature = "tmux_2_0")]
        if self.append {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-w]`
        #[cfg(feature = "tmux_3_2")]
        if self.send_to_clipboard {
            cmd.push_flag(W_LOWERCASE_KEY);
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

        // `[-n new-buffer-name]`
        #[cfg(feature = "tmux_2_0")]
        if let Some(new_buffer_name) = self.new_buffer_name {
            cmd.push_option(N_LOWERCASE_KEY, new_buffer_name);
        }

        // `data`
        #[cfg(feature = "tmux_0_8")]
        if let Some(data) = self.data {
            cmd.push_param(data);
        }

        // `[-b buffer-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
        if let Some(buffer_index) = self.buffer_index {
            cmd.push_option(B_LOWERCASE_KEY, buffer_index);
        }

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
