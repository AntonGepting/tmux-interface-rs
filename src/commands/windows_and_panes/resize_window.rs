use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Resize a window, up, down, left or right
///
/// # Manual
///
/// tmux ^2.9:
/// ```text
/// tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
/// (alias: resizew)
/// ```
#[derive(Debug, Clone)]
pub struct ResizeWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ResizeWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(RESIZE_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> ResizeWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - set the size of the smallest session containing the window
    pub fn smallest(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-A]` - set the size of the largest session containing the window
    pub fn largest(&mut self) -> &mut Self {
        self.0.push_flag(A_UPPERCASE_KEY);
        self
    }

    /// `[-D]` - resize down by adjustment
    pub fn down(&mut self) -> &mut Self {
        self.0.push_flag(D_UPPERCASE_KEY);
        self
    }

    /// `[-L]` - resize left by adjustment
    pub fn left(&mut self) -> &mut Self {
        self.0.push_flag(L_UPPERCASE_KEY);
        self
    }

    /// `[-R]` - resize right by adjustment
    pub fn right(&mut self) -> &mut Self {
        self.0.push_flag(R_UPPERCASE_KEY);
        self
    }

    /// `[-U]` - resize up by adjustment
    pub fn up(&mut self) -> &mut Self {
        self.0.push_flag(U_UPPERCASE_KEY);
        self
    }

    /// `[-t target-window]` - target-window
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `[-x width]` - absolute size
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.0.push_option(X_LOWERCASE_KEY, width.to_string());
        self
    }

    /// `[-y height]` - absolute size
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.0.push_option(Y_LOWERCASE_KEY, height.to_string());
        self
    }

    /// `[adjustment]` - adjustment
    pub fn adjustment<S: Into<Cow<'a, str>>>(&mut self, adjustment: S) -> &mut Self {
        self.0.push_param(adjustment);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ResizeWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(RESIZE_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ResizeWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(RESIZE_WINDOW)),
            ..Default::default()
        })
    }
}
