use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Resize a window, up, down, left or right
///
/// # Manual
///
/// tmux ^2.9:
/// ```text
/// resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
/// (alias: resizew)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ResizeWindow<'a> {
    /// `[-a]` - set the size of the smallest session containing the window
    pub smallest: bool,

    /// `[-A]` - set the size of the largest session containing the window
    pub largest: bool,

    /// `[-D]` - resize down by adjustment
    pub down: bool,

    /// `[-L]` - resize left by adjustment
    pub left: bool,

    /// `[-R]` - resize right by adjustment
    pub right: bool,

    /// `[-U]` - resize up by adjustment
    pub up: bool,

    /// `[-t target-window]` - target-window
    pub target_window: Option<Cow<'a, str>>,

    /// `[-x width]` - absolute size
    pub width: Option<usize>,

    /// `[-y height]` - absolute size
    pub height: Option<usize>,

    /// `[adjustment]` - adjustment
    pub adjustment: Option<Cow<'a, str>>,
}

impl<'a> ResizeWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - set the size of the smallest session containing the window
    pub fn smallest(mut self) -> Self {
        self.smallest = true;
        self
    }

    /// `[-A]` - set the size of the largest session containing the window
    pub fn largest(mut self) -> Self {
        self.largest = true;
        self
    }

    /// `[-D]` - resize down by adjustment
    pub fn down(mut self) -> Self {
        self.down = true;
        self
    }

    /// `[-L]` - resize left by adjustment
    pub fn left(mut self) -> Self {
        self.left = true;
        self
    }

    /// `[-R]` - resize right by adjustment
    pub fn right(mut self) -> Self {
        self.right = true;
        self
    }

    /// `[-U]` - resize up by adjustment
    pub fn up(mut self) -> Self {
        self.up = true;
        self
    }

    /// `[-t target-window]` - target-window
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-x width]` - absolute size
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// `[-y height]` - absolute size
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// `[adjustment]` - adjustment
    pub fn adjustment<S: Into<Cow<'a, str>>>(mut self, adjustment: S) -> Self {
        self.adjustment = Some(adjustment.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(RESIZE_WINDOW);

        // `[-a]` - set the size of the smallest session containing the window
        if self.smallest {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-A]` - set the size of the largest session containing the window
        if self.largest {
            cmd.push_flag(A_UPPERCASE_KEY);
        }

        // `[-D]` - resize down by adjustment
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-L]` - resize left by adjustment
        if self.left {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-R]` - resize right by adjustment
        if self.right {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-U]` - resize up by adjustment
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-t target-window]` - target-window
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-x width]` - absolute size
        if let Some(width) = self.width {
            cmd.push_option(X_LOWERCASE_KEY, width.to_string());
        }

        // `[-y height]` - absolute size
        if let Some(height) = self.height {
            cmd.push_option(Y_LOWERCASE_KEY, height.to_string());
        }

        // `[adjustment]` - adjustment
        if let Some(adjustment) = self.adjustment {
            cmd.push_param(adjustment);
        }

        cmd
    }
}
