use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type DeleteB<'a> = DeleteBuffer<'a>;

/// Delete the buffer named buffer-name, or the most recently added automatically named buffer
/// if not specified.
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// delete-buffer [-b buffer-name]
/// (alias: deleteb)
/// ```
///
/// tmux ^1.5 v2.0:
/// ```text
/// delete-buffer [-b buffer-index]
/// (alias: deleteb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// delete-buffer [-b buffer-index] [-t target-session]
/// (alias: deleteb)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DeleteBuffer<'a> {
    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub buffer_name: Option<Cow<'a, str>>,

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    pub buffer_index: Option<Cow<'a, str>>,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> DeleteBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(mut self, buffer_name: S) -> Self {
        self.buffer_name = Some(buffer_name.into());
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
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

        cmd.name(DELETE_BUFFER);

        // `[-b buffer-name]`
        #[cfg(feature = "tmux_2_0")]
        if let Some(buffer_name) = self.buffer_name {
            cmd.push_option(B_LOWERCASE_KEY, buffer_name);
        }

        // `[-b buffer-index]`
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
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
