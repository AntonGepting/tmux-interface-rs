// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type ResizeP<'a> = ResizePane<'a>;

/// Resize a pane, up, down, left or right
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// resize-pane [-DLMRTUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux >=2.1:
/// ```text
/// resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux >=1.8:
/// ```text
/// resize-pane [-DLRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux >=1.5:
/// ```text
/// resize-pane [-DLRU] [-t target-pane] [adjustment]
/// (alias: resizep)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ResizePane<'a> {
    /// `[-D]`
    #[cfg(feature = "tmux_1_5")]
    pub down: bool,

    /// `[-L]`
    #[cfg(feature = "tmux_1_5")]
    pub left: bool,

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub mouse: bool,

    /// `[-R]`
    #[cfg(feature = "tmux_1_5")]
    pub right: bool,

    /// `[-T]`
    #[cfg(feature = "tmux_3_2")]
    pub trim: bool,

    /// `[-U]`
    #[cfg(feature = "tmux_1_5")]
    pub up: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_1_8")]
    pub zoom: bool,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-x width]`
    #[cfg(feature = "tmux_1_8")]
    pub width: Option<Cow<'a, str>>,

    /// `[-y height]`
    #[cfg(feature = "tmux_1_8")]
    pub height: Option<Cow<'a, str>>,

    /// `[adjustment]`
    #[cfg(feature = "tmux_1_5")]
    pub adjustment: Option<Cow<'a, str>>,
}

impl<'a> ResizePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-D]`
    #[cfg(feature = "tmux_1_5")]
    pub fn down(mut self) -> Self {
        self.down = true;
        self
    }

    /// `[-L]`
    #[cfg(feature = "tmux_1_5")]
    pub fn left(mut self) -> Self {
        self.left = true;
        self
    }

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(mut self) -> Self {
        self.mouse = true;
        self
    }

    /// `[-R]`
    #[cfg(feature = "tmux_1_5")]
    pub fn right(mut self) -> Self {
        self.right = true;
        self
    }

    /// `[-T]`
    #[cfg(feature = "tmux_3_2")]
    pub fn trim(mut self) -> Self {
        self.trim = true;
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_1_5")]
    pub fn up(mut self) -> Self {
        self.up = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_1_8")]
    pub fn zoom(mut self) -> Self {
        self.zoom = true;
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-x width]`
    #[cfg(feature = "tmux_1_8")]
    pub fn width<S: Into<Cow<'a, str>>>(mut self, width: S) -> Self {
        self.width = Some(width.into());
        self
    }

    /// `[-y height]`
    #[cfg(feature = "tmux_1_8")]
    pub fn height<S: Into<Cow<'a, str>>>(mut self, height: S) -> Self {
        self.height = Some(height.into());
        self
    }

    /// `[adjustment]`
    #[cfg(feature = "tmux_1_5")]
    pub fn adjustment<S: Into<Cow<'a, str>>>(mut self, adjustment: S) -> Self {
        self.adjustment = Some(adjustment.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(RESIZE_PANE);

        // `[-D]`
        #[cfg(feature = "tmux_1_5")]
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-L]`
        #[cfg(feature = "tmux_1_5")]
        if self.left {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-M]`
        #[cfg(feature = "tmux_2_1")]
        if self.mouse {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-R]`
        #[cfg(feature = "tmux_1_5")]
        if self.right {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-T]`
        #[cfg(feature = "tmux_3_2")]
        if self.trim {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-U]`
        #[cfg(feature = "tmux_1_5")]
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_1_8")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-x width]`
        #[cfg(feature = "tmux_1_8")]
        if let Some(width) = self.width {
            cmd.push_option(X_LOWERCASE_KEY, width);
        }

        // `[-y height]`
        #[cfg(feature = "tmux_1_8")]
        if let Some(height) = self.height {
            cmd.push_option(Y_LOWERCASE_KEY, height);
        }

        // `[adjustment]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(adjustment) = self.adjustment {
            cmd.push_param(adjustment);
        }

        cmd
    }
}
