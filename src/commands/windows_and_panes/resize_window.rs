// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type ResizeW<'a> = ResizeWindow<'a>;

/// Resize a window, up, down, left or right
///
/// # Manual
///
/// tmux >=2.9:
/// ```text
/// resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
/// (alias: resizew)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ResizeWindow<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_2_9")]
    pub smallest: bool,

    /// `[-A]`
    #[cfg(feature = "tmux_2_9")]
    pub largest: bool,

    /// `[-D]`
    #[cfg(feature = "tmux_2_9")]
    pub down: bool,

    /// `[-L]`
    #[cfg(feature = "tmux_2_9")]
    pub left: bool,

    /// `[-R]`
    #[cfg(feature = "tmux_2_9")]
    pub right: bool,

    /// `[-U]`
    #[cfg(feature = "tmux_2_9")]
    pub up: bool,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_2_9")]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-x width]`
    #[cfg(feature = "tmux_2_9")]
    pub width: Option<usize>,

    /// `[-y height]`
    #[cfg(feature = "tmux_2_9")]
    pub height: Option<usize>,

    /// `[adjustment]`
    #[cfg(feature = "tmux_2_9")]
    pub adjustment: Option<Cow<'a, str>>,
}

impl<'a> ResizeWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_2_9")]
    pub fn smallest(mut self) -> Self {
        self.smallest = true;
        self
    }

    /// `[-A]`
    #[cfg(feature = "tmux_2_9")]
    pub fn largest(mut self) -> Self {
        self.largest = true;
        self
    }

    /// `[-D]`
    #[cfg(feature = "tmux_2_9")]
    pub fn down(mut self) -> Self {
        self.down = true;
        self
    }

    /// `[-L]`
    #[cfg(feature = "tmux_2_9")]
    pub fn left(mut self) -> Self {
        self.left = true;
        self
    }

    /// `[-R]`
    #[cfg(feature = "tmux_2_9")]
    pub fn right(mut self) -> Self {
        self.right = true;
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_2_9")]
    pub fn up(mut self) -> Self {
        self.up = true;
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_2_9")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-x width]`
    #[cfg(feature = "tmux_2_9")]
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// `[-y height]`
    #[cfg(feature = "tmux_2_9")]
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// `[adjustment]`
    #[cfg(feature = "tmux_2_9")]
    pub fn adjustment<S: Into<Cow<'a, str>>>(mut self, adjustment: S) -> Self {
        self.adjustment = Some(adjustment.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(RESIZE_WINDOW);

        // `[-a]`
        #[cfg(feature = "tmux_2_9")]
        if self.smallest {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-A]`
        #[cfg(feature = "tmux_2_9")]
        if self.largest {
            cmd.push_flag(A_UPPERCASE_KEY);
        }

        // `[-D]`
        #[cfg(feature = "tmux_2_9")]
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-L]`
        #[cfg(feature = "tmux_2_9")]
        if self.left {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-R]`
        #[cfg(feature = "tmux_2_9")]
        if self.right {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-U]`
        #[cfg(feature = "tmux_2_9")]
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_2_9")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-x width]`
        #[cfg(feature = "tmux_2_9")]
        if let Some(width) = self.width {
            cmd.push_option(X_LOWERCASE_KEY, width.to_string());
        }

        // `[-y height]`
        #[cfg(feature = "tmux_2_9")]
        if let Some(height) = self.height {
            cmd.push_option(Y_LOWERCASE_KEY, height.to_string());
        }

        // `[adjustment]`
        #[cfg(feature = "tmux_2_9")]
        if let Some(adjustment) = self.adjustment {
            cmd.push_param(adjustment);
        }

        cmd
    }
}
