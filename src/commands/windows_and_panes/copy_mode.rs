use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Enter copy mode
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux copy-mode [-Meu] [-t target-pane]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux copy-mode [-u] [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux copy-mode [-u] [-t target-window]
/// ```
#[derive(Debug, Clone)]
pub struct CopyMode<'a>(pub TmuxCommand<'a>);

impl<'a> Default for CopyMode<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(COPY_MODE)),
            ..Default::default()
        })
    }
}

impl<'a> CopyMode<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse_drag(&mut self) -> &mut Self {
        self.0.push_flag(M_UPPERCASE_KEY);
        self
    }

    /// `[-e]`
    #[cfg(feature = "tmux_2_1")]
    pub fn bottom_exit(&mut self) -> &mut Self {
        self.0.push_flag(E_LOWERCASE_KEY);
        self
    }

    /// `[-u]`
    #[cfg(feature = "tmux_0_8")]
    pub fn page_up(&mut self) -> &mut Self {
        self.0.push_flag(U_LOWERCASE_KEY);
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for CopyMode<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(COPY_MODE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for CopyMode<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(COPY_MODE)),
            ..Default::default()
        })
    }
}
