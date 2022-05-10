use crate::commands::constants::*;
use crate::TmuxCommand;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RotateWindow<'a> {
    /// `[-D]`
    #[cfg(feature = "tmux_0_8")]
    pub down: bool,

    /// `[-U]`
    #[cfg(feature = "tmux_0_8")]
    pub up: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: bool,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> RotateWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-D]`
    #[cfg(feature = "tmux_0_8")]
    pub fn down(&mut self) -> &mut Self {
        self.down = true;
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_0_8")]
    pub fn up(&mut self) -> &mut Self {
        self.up = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = true;
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(ROTATE_WINDOW);

        // `[-D]`
        #[cfg(feature = "tmux_0_8")]
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-U]`
        #[cfg(feature = "tmux_0_8")]
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_3_1")]
        if self.keep_zoomed {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = &self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window.as_ref());
        }

        cmd
    }
}
