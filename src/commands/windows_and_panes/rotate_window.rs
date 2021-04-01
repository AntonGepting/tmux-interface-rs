use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Rotate the positions of the panes within a window
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux rotate-window [-DUZ] [-t target-window]
/// (alias: rotatew)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux rotate-window [-DU] [-t target-window]
/// (alias: rotatew)
/// ```
#[derive(Debug, Clone)]
pub struct RotateWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for RotateWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(ROTATE_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> RotateWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-D]`
    #[cfg(feature = "tmux_0_8")]
    pub fn down(&mut self) -> &mut Self {
        self.0.push_flag(D_UPPERCASE_KEY);
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_0_8")]
    pub fn up(&mut self) -> &mut Self {
        self.0.push_flag(U_UPPERCASE_KEY);
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for RotateWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(ROTATE_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for RotateWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(ROTATE_WINDOW)),
            ..Default::default()
        })
    }
}
