use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Select the window at target-window.
///
/// # Manual
///
/// tmux ^1.8:
/// ```text
/// tmux select-window [-lnpT] [-t target-window]
/// (alias: selectw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux select-window [-lnp] [-t target-window]
/// (alias: selectw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux select-window [-t target-window]
/// (alias: selectw)
/// ```
#[derive(Debug, Default, Clone)]
pub struct SelectWindow<'a> {
    /// `[-l]` - equivalent to last-window
    #[cfg(feature = "tmux_1_5")]
    pub last: bool,

    /// `[-n]` - equivalent to next-window
    #[cfg(feature = "tmux_1_5")]
    pub next: bool,

    /// `[-p]` - equivalent to previous-window
    #[cfg(feature = "tmux_1_5")]
    pub previous: bool,

    /// `[-T]` - if the selected window is already the current window, behave like last-window
    #[cfg(feature = "tmux_1_8")]
    pub switch: bool,

    /// `[-t target-window]` - target-window
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> SelectWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-l]` - equivalent to last-window
    #[cfg(feature = "tmux_1_5")]
    pub fn last(&mut self) -> &mut Self {
        self.last = true;
        self
    }

    /// `[-n]` - equivalent to next-window
    #[cfg(feature = "tmux_1_5")]
    pub fn next(&mut self) -> &mut Self {
        self.next = true;
        self
    }

    /// `[-p]` - equivalent to previous-window
    #[cfg(feature = "tmux_1_5")]
    pub fn previous(&mut self) -> &mut Self {
        self.previous = true;
        self
    }

    /// `[-T]` - if the selected window is already the current window, behave like last-window
    #[cfg(feature = "tmux_1_8")]
    pub fn switch(&mut self) -> &mut Self {
        self.switch = true;
        self
    }

    /// `[-t target-window]` - target-window
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SELECT_WINDOW);

        // `[-l]` - equivalent to last-window
        #[cfg(feature = "tmux_1_5")]
        if self.last {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-n]` - equivalent to next-window
        #[cfg(feature = "tmux_1_5")]
        if self.next {
            cmd.push_flag(N_LOWERCASE_KEY);
        }

        // `[-p]` - equivalent to previous-window
        #[cfg(feature = "tmux_1_5")]
        if self.previous {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-T]` - if the selected window is already the current window, behave like last-window
        #[cfg(feature = "tmux_1_8")]
        if self.switch {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-t target-window]` - target-window
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = &self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window.as_ref());
        }

        cmd
    }
}
