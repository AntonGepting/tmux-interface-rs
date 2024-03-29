use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type ShowB<'a> = ShowBuffer<'a>;

/// Display the contents of the specified buffer.
///
/// # Manual
///
/// tmux ^1.5:
/// ```text
/// show-buffer [-b buffer-name]
/// (alias: showb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// show-buffer [-b buffer-index] [-t target-session]
/// (alias: showb)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ShowBuffer<'a> {
    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_1_5")]
    pub buffer_name: Option<Cow<'a, str>>,

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub buffer_index: Option<Cow<'a, str>>,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> ShowBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(mut self, buffer_name: S) -> Self {
        self.buffer_name = Some(buffer_name.into());
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
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

        cmd.name(SHOW_BUFFER);

        // `[-b buffer-name]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(buffer_name) = self.buffer_name {
            cmd.push_option(B_LOWERCASE_KEY, buffer_name);
        }

        // `[-b buffer-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
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
