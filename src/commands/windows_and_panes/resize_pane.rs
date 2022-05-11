use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Resize a pane, up, down, left or right
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux resize-pane [-DLMRTUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ResizePane<'a> {
    /// `[-D]` - resize down by adjustment
    #[cfg(feature = "tmux_0_9")]
    pub down: bool,

    /// `[-L]` - resize left by adjustment
    #[cfg(feature = "tmux_1_8")]
    pub left: bool,

    /// `[-M]` - begin mouse resizing
    #[cfg(feature = "tmux_2_1")]
    pub mouse: bool,

    /// `[-R]` - resize right by adjustment
    #[cfg(feature = "tmux_1_8")]
    pub right: bool,

    /// `[-T]` - trims all lines below the current cursor position
    #[cfg(feature = "tmux_3_2")]
    pub trim: bool,

    /// `[-U]` - resize up by adjustment
    #[cfg(feature = "tmux_0_9")]
    pub up: bool,

    /// `[-Z]` - the active pane is toggled between zoomed and unzoomed
    #[cfg(feature = "tmux_1_8")]
    pub zoom: bool,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_9")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-x width]` - absolute size
    #[cfg(feature = "tmux_1_8")]
    pub width: Option<usize>,

    /// `[-y height]` - absolute size
    #[cfg(feature = "tmux_1_8")]
    pub height: Option<usize>,

    /// `[adjustment]` - adjustment
    #[cfg(feature = "tmux_0_9")]
    pub adjustment: Option<Cow<'a, str>>,
}

impl<'a> ResizePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-D]` - resize down by adjustment
    #[cfg(feature = "tmux_0_9")]
    pub fn down(&mut self) -> &mut Self {
        self.down = true;
        self
    }

    /// `[-L]` - resize left by adjustment
    #[cfg(feature = "tmux_1_8")]
    pub fn left(&mut self) -> &mut Self {
        self.left = true;
        self
    }

    /// `[-M]` - begin mouse resizing
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(&mut self) -> &mut Self {
        self.mouse = true;
        self
    }

    /// `[-R]` - resize right by adjustment
    #[cfg(feature = "tmux_1_8")]
    pub fn right(&mut self) -> &mut Self {
        self.right = true;
        self
    }

    /// `[-T]` - trims all lines below the current cursor position
    #[cfg(feature = "tmux_3_2")]
    pub fn trim(&mut self) -> &mut Self {
        self.trim = true;
        self
    }

    /// `[-U]` - resize up by adjustment
    #[cfg(feature = "tmux_0_9")]
    pub fn up(&mut self) -> &mut Self {
        self.up = true;
        self
    }

    /// `[-Z]` - the active pane is toggled between zoomed and unzoomed
    #[cfg(feature = "tmux_1_8")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = true;
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_9")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-x width]` - absolute size
    #[cfg(feature = "tmux_1_8")]
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }

    /// `[-y height]` - absolute size
    #[cfg(feature = "tmux_1_8")]
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.height = Some(height);
        self
    }

    /// `[adjustment]` - adjustment
    #[cfg(feature = "tmux_0_9")]
    pub fn adjustment<S: Into<Cow<'a, str>>>(&mut self, adjustment: S) -> &mut Self {
        self.adjustment = Some(adjustment.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(RESIZE_PANE);

        // `[-D]` - resize down by adjustment
        #[cfg(feature = "tmux_0_9")]
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-L]` - resize left by adjustment
        #[cfg(feature = "tmux_1_8")]
        if self.left {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-M]` - begin mouse resizing
        #[cfg(feature = "tmux_2_1")]
        if self.mouse {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-R]` - resize right by adjustment
        #[cfg(feature = "tmux_1_8")]
        if self.right {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-T]` - trims all lines below the current cursor position
        #[cfg(feature = "tmux_3_2")]
        if self.trim {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-U]` - resize up by adjustment
        #[cfg(feature = "tmux_0_9")]
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-Z]` - the active pane is toggled between zoomed and unzoomed
        #[cfg(feature = "tmux_1_8")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_0_9")]
        if let Some(target_pane) = &self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane.as_ref());
        }

        // `[-x width]` - absolute size
        #[cfg(feature = "tmux_1_8")]
        if let Some(width) = &self.width {
            cmd.push_option(X_LOWERCASE_KEY, width.to_string());
        }

        // `[-y height]` - absolute size
        #[cfg(feature = "tmux_1_8")]
        if let Some(height) = &self.height {
            cmd.push_option(Y_LOWERCASE_KEY, height.to_string());
        }

        // `[adjustment]` - adjustment
        #[cfg(feature = "tmux_0_9")]
        if let Some(adjustment) = &self.adjustment {
            cmd.push_param(adjustment.as_ref());
        }

        cmd
    }
}
