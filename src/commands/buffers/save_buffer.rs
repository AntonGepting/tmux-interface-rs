use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Save the contents of the specified paste buffer to path.
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// save-buffer [-a] [-b buffer-name] path
/// (alias: saveb)
/// ```
///
/// tmux ^1.5:
/// ```text
/// save-buffer [-a] [-b buffer-index] path
/// (alias: saveb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// save-buffer [-a] [-b buffer-index] [-t target-session] path
/// (alias: saveb)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SaveBuffer<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_0_8")]
    pub append: bool,

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub buffer_name: Option<Cow<'a, str>>,

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

impl<'a> SaveBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_0_8")]
    pub fn append(mut self) -> Self {
        self.append = true;
        self
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(mut self, buffer_name: S) -> Self {
        self.buffer_name = Some(buffer_name.into());
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

    /// `[path]`
    #[cfg(feature = "tmux_0_8")]
    pub fn path<S: Into<Cow<'a, str>>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SAVE_BUFFER);

        // `[-a]`
        #[cfg(feature = "tmux_0_8")]
        if self.append {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-b buffer-name]`
        #[cfg(feature = "tmux_2_0")]
        if let Some(buffer_name) = self.buffer_name {
            cmd.push_option(B_LOWERCASE_KEY, buffer_name);
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

        // `[path]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(path) = self.path {
            cmd.push_param(path);
        }

        cmd
    }
}
