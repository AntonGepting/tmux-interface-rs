use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Resize a pane, up, down, left or right
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux resize-pane [-DLRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux resize-pane [-DLRU] [-t target-pane] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux resize-pane [-DU] [-p pane-index] [-t target-pane] [adjustment]
/// (alias: resizep)
/// ```
#[derive(Debug, Clone)]
pub struct ResizePane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ResizePane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(RESIZE_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> ResizePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-D]` - resize down by adjustment
    #[cfg(feature = "tmux_0_9")]
    pub fn down(&mut self) -> &mut Self {
        self.0.push_flag(D_UPPERCASE_KEY);
        self
    }

    /// `[-L]` - resize left by adjustment
    #[cfg(feature = "tmux_1_8")]
    pub fn left(&mut self) -> &mut Self {
        self.0.push_flag(L_UPPERCASE_KEY);
        self
    }

    /// `[-M]` - begin mouse resizing
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(&mut self) -> &mut Self {
        self.0.push_flag(M_UPPERCASE_KEY);
        self
    }

    /// `[-R]` - resize right by adjustment
    #[cfg(feature = "tmux_1_8")]
    pub fn right(&mut self) -> &mut Self {
        self.0.push_flag(R_UPPERCASE_KEY);
        self
    }

    /// `[-U]` - resize up by adjustment
    #[cfg(feature = "tmux_0_9")]
    pub fn up(&mut self) -> &mut Self {
        self.0.push_flag(U_UPPERCASE_KEY);
        self
    }

    /// `[-Z]` - the active pane is toggled between zoomed and unzoomed
    #[cfg(feature = "tmux_1_8")]
    pub fn zoom(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_9")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-x width]` - absolute size
    #[cfg(feature = "tmux_1_8")]
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.0.push_option(X_LOWERCASE_KEY, width.to_string());
        self
    }

    /// `[-y height]` - absolute size
    #[cfg(feature = "tmux_1_8")]
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.0.push_option(Y_LOWERCASE_KEY, height.to_string());
        self
    }

    /// `[adjustment]` - adjustment
    #[cfg(feature = "tmux_0_9")]
    pub fn adjustment<S: Into<Cow<'a, str>>>(&mut self, adjustment: S) -> &mut Self {
        self.0.push_param(adjustment);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ResizePane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(RESIZE_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ResizePane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(RESIZE_PANE)),
            ..Default::default()
        })
    }
}
