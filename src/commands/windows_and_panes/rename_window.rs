use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Rename the current window, or the window at target-window if specified, to new-name
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux rename-window [-t target-window] new-name
/// (alias: renamew)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RenameWindow<'a> {
    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,

    /// `new-name`
    #[cfg(feature = "tmux_0_8")]
    pub new_name: Option<Cow<'a, str>>,
}

impl<'a> RenameWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `new-name`
    #[cfg(feature = "tmux_0_8")]
    pub fn new_name<S: Into<Cow<'a, str>>>(mut self, new_name: S) -> Self {
        self.new_name = Some(new_name.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(RENAME_WINDOW);

        // `[-t target-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `new-name`
        #[cfg(feature = "tmux_0_8")]
        if let Some(new_name) = self.new_name {
            cmd.push_param(new_name);
        }
        cmd
    }
}
