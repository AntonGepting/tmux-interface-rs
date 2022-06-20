use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Select the last (previously selected) pane
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux last-pane [-deZ] [-t target-window]
/// (alias: lastp)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux last-pane [-de] [-t target-window]
/// (alias: lastp)
/// ```
///
/// tmux ^1.4:
/// ```text
/// tmux last-pane [-t target-window]
/// (alias: lastp)
/// ```
// FIXME: versions and function parameters
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LastPane<'a> {
    /// `[-d]`
    #[cfg(feature = "tmux_2_0")]
    pub disable: bool,

    /// `[-e]`
    #[cfg(feature = "tmux_2_0")]
    pub enable: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: bool,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_1_4")]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> LastPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]`
    #[cfg(feature = "tmux_2_0")]
    pub fn disable(mut self) -> Self {
        self.disable = true;
        self
    }

    /// `[-e]`
    #[cfg(feature = "tmux_2_0")]
    pub fn enable(mut self) -> Self {
        self.enable = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(mut self) -> Self {
        self.keep_zoomed = true;
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_1_4")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LAST_PANE);

        // `[-d]`
        #[cfg(feature = "tmux_2_0")]
        if self.disable {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-e]`
        #[cfg(feature = "tmux_2_0")]
        if self.enable {
            cmd.push_flag(E_LOWERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_3_1")]
        if self.keep_zoomed {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_1_4")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
