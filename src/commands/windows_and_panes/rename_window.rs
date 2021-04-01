use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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
#[derive(Debug, Clone)]
pub struct RenameWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for RenameWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(RENAME_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> RenameWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `new-name`
    #[cfg(feature = "tmux_0_8")]
    pub fn new_name<S: Into<Cow<'a, str>>>(&mut self, new_name: S) -> &mut Self {
        self.0.push_param(new_name);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for RenameWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(RENAME_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for RenameWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(RENAME_WINDOW)),
            ..Default::default()
        })
    }
}
